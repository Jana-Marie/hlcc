import { compute_input } from "../pkg/hlcc.js";

var textBox = document.getElementById('hormone_input');
textBox.addEventListener('keypress', function(){
    if(event.which == 13){
      let res = compute_input(document.getElementById('hormone_input').value);
      document.getElementById("hormone_output").innerHTML = res
      document.getElementById("hormone_output").classList.add("fade-in");
      console.log(res);
    }
});
