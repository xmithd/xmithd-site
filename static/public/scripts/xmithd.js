'use strict';

const DEBUG_MODE = false;

function debugln(msg) {
  const debug = document.getElementById('debug');
  if (debug && DEBUG_MODE)
    debug.innerHTML += (msg) + '<br/>\n';
}

document.addEventListener('DOMContentLoaded', function () {
  const dateElements = document.getElementsByClassName('xm_timestamp');
  debugln("size = " + dateElements.length);
  for (const element of dateElements) {
    const numstr = element.innerHTML;
    debugln("numstr = " + numstr);
    const value = parseInt(numstr, 10);
    debugln("value: " + value);
    if (!isNaN(value)) {
      let dateVal = new Date(value);
      // Quick way to display local date
      debugln("dateVal " + dateVal.toISOString() );
      element.innerHTML = dateVal.toLocaleDateString('zu');
    }
  }
});