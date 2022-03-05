import { compute_input } from "../pkg/hlcc.js";

var textBox = document.getElementById('hormone_input');
textBox.addEventListener('keypress', function(){
    if(event.which == 13){
      let res = compute_input(document.getElementById('hormone_input').value);
      if (res != -1.0){
        document.getElementById("hormone_output").innerHTML = "computes to: " + Math.round((res + Number.EPSILON) * 1000) / 1000;
      } else {
        document.getElementById("hormone_output").innerHTML = "Input error!";
      }
      document.getElementById("hormone_output").classList.add("fade-in");
      console.log(res);
    }
});
