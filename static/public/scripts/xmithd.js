'use strict';

const DEBUG_MODE = false;

function debugln(msg) {
  const debug = document.getElementById('debug');
  if (debug && DEBUG_MODE)
    debug.innerHTML += (msg) + '<br/>\n';
}

function displayLocalDates() {
  const dateElements = document.getElementsByClassName('xm_timestamp');
  debugln("local dates size = " + dateElements.length);
  for (const element of dateElements) {
    let timestampStr = element.innerHTML;
    const localeIdx = timestampStr.indexOf('-');
    let locale = undefined;
    if (localeIdx !== -1) {
      locale = timestampStr.substr(0, localeIdx);
      debugln('locale = ' + locale );
      timestampStr = timestampStr.substr(localeIdx+1, timestampStr.length-localeIdx+1);
    }
    debugln("timestampStr = " + timestampStr);
    const value = parseInt(timestampStr, 10);
    debugln("numeric value: " + value);
    if (!isNaN(value)) {
      let dateVal = new Date(value);
      // Quick way to display local date
      if (locale) {
        element.innerHTML = dateVal.toLocaleDateString('zu');
      } else {
        element.innerHTML = dateVal.toString();
      }
    }
  }

}

function setUpEchoFunction() {
  var conn = null;
  function log(msg) {
    var control = document.getElementById('log');
    if (control) {
      // unsafe but it's only for testing...
      control.innerHTML += msg + '<br />';
      control.scrollBy(0, 1000);
    } else {
      debugln("websocket control element not found.");
    }
  }
  function update_ui() {
    let msg = '';
    const statusEl = document.getElementById('status');
    const connectEl = document.getElementById('connect');
    if (statusEl && connectEl) {
      if (conn === null) {
        statusEl.innerText = 'disconnected';
        connectEl.innerHTML = 'Connect';
      } else {
        statusEl.innerText = 'connected (' + conn.protocol + ')';
        connectEl.innerHTML = 'Disconnect'
      }
    }
  }
  function disconnect() {
    if (conn !== null) {
      log('Disconnecting...');
      conn.close();
      conn = null;
      update_ui();
    }
  }
  function connect() {
    disconnect();
    const wsUri = (window.location.protocol==='https:'&&'wss://'||'ws://')+ window.location.host + '/ws';
    conn = new WebSocket(wsUri);
    log('Connecting...');
    conn.onopen = function() {
      log('Connected.');
      update_ui();
    };
    conn.onmessage = function(e) {
      log('Received: ' + e.data);
    }
    conn.onclose = function() {
      log('Disconnected.');
      conn = null;
      update_ui();
    }
  }
  const connectBtn = document.getElementById('connect');
  const textEl = document.getElementById('text');
  if (connectBtn) {
    connectBtn.addEventListener('click', () => {
       if (conn === null) {
         connect();
       } else {
         disconnect();
       }
       update_ui();
       return false;
    });
  }
  const sendBtn = document.getElementById('send');
  if (sendBtn) {
    sendBtn.addEventListener('click', () => {
      let text = textEl.value;
      debugln('text is ' + text);
      log('Sending: ' + text);
      conn.send(text);
      textEl.value = '';
      textEl.focus();
      return false;
    });
  }
  if (textEl) {
    textEl.addEventListener('keyup', (e) => {
      if (e.code === 'Enter') {
        sendBtn.click();
        return false;
      }
    });
  }
}

document.addEventListener('DOMContentLoaded', function () {
  displayLocalDates();
  setUpEchoFunction();
});
