import { compute_input } from "../pkg/hlcc.js";

var textBox = document.getElementById('hormone_input');
textBox.addEventListener('keypress', function(){
    if(event.which == 13){
      let res = compute_input(document.getElementById('hormone_input').value);
      if (res != -1.0){
        document.getElementById("hormone_output").innerHTML = "computes to: " + Math.round((res + Number.EPSILON) * 100) / 100;
      } else {
        document.getElementById("hormone_output").innerHTML = "Input error!";
      }
      //document.getElementById("hormone_output").classList.remove("fade-out");
      document.getElementById("hormone_output").classList.add("fade-in");
      console.log(res);
    }
    else if(event.which == 8 || event.which == 13){ //if (document.getElementById('hormone_input').value == "")  // why does this not work :(
      document.getElementById("hormone_output").classList.add("fade-out");
      document.getElementById("hormone_output").classList.remove("fade-in");
    }
});
