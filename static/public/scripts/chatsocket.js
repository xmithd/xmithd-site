function setUpEchoFunction() {
    let conn = null;
    function log(msg) {
        var control = document.getElementById('log');
        if (control) {
            // unsafe but it's only for testing...
            control.innerHTML += msg + '<br />';
            control.scrollBy(0, 1000);
        }
    }
    function update_ui() {
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
        };
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
    setUpEchoFunction();
});