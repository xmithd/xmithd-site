'use strict';

const DEBUG_MODE = false;

function debugln(msg) {
  const debug = document.getElementById('debug');
  if (debug && DEBUG_MODE)
    debug.innerHTML += (msg) + '<br/>\n';
}

function displayLocalDates() {
  const dateElements = document.getElementsByClassName('xm_timestamp');
  debugln("size = " + dateElements.length);
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

document.addEventListener('DOMContentLoaded', function () {
  displayLocalDates();
});