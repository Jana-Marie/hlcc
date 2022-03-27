import { compute_input } from "../pkg/hlcc_web.js";

if(window.location.hash) {
  process_fragment();
}

window.addEventListener('hashchange', process_fragment);

document.getElementsByTagName('form')[0].addEventListener('submit', function(ev){
  window.location.hash = document.getElementsByTagName('input')[0].value;
  ev.preventDefault();
});

function process_fragment(){
  let input = decodeURI(window.location.hash.substring(1));
  document.getElementsByTagName('input')[0].value = input;
  let out = document.getElementById("hormone_output");
  let res = compute_input(input);
  out.innerHTML = res;
  out.classList.add("fade-in");
  console.log(res);

}
