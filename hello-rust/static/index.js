$(document).ready(function() {		
    $("#driver").click(function(event){
       $.getJSON('/config', function(jd) {
          $('#td1').html(jd.name);
          $('#td2').html(jd.age);
          $('#td3').html(jd.status);
          $('#td4').html(jd.github);
       });
    }); 
    // I feel a bit dirty writing in jQuery again.  
 });