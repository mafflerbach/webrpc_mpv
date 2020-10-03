$(function() {
  get_volume();

  // unmute sound
  $("#volume-unmute").click(function() {
    $("#volume-unmute").hide();
    $("#volume-mute").show();
    selectedClient = $("#clientSelection").val();
    let value = $("#video-volume input[name='range']").data("before");
    $.ajax({
      type : "POST",
      url : "/volume",
      contenType : "application/json",
      data : JSON.stringify({"value" : value, "client" : selectedClient}),
      success : function(data) {
        $("#video-volume-output").html(value);
        $("#video-volume input[name='range']").val(value);
      },
      dataType : 'json'
    });
  });

  // mute sound
  $("#volume-mute").click(function() {
    selectedClient = $("#clientSelection").val();
    $("#volume-unmute").show();
    $("#volume-mute").hide();
    $.ajax({
      type : "POST",
      url : "/volume",
      contenType : "application/json",
      data : JSON.stringify({"value" : "0", "client" : selectedClient}),
      success : function(data) {
        let value = $("#video-volume input[name='range']").val();
        $("#video-volume input[name='range']").data("before", value);
        $("#video-volume-output").html(value);
        $("#video-volume input[name='range']").val(0);
      },
      dataType : 'json'
    });
  });

  var myInterval = setInterval(function() { clearInterval(myInterval); }, 2000);

  document.addEventListener('keydown', (e) => {
    if (e.code == "VolumeDown") {
      volume_change(-5);
    } else if (e.code == "VolumeUp") {
      volume_change(5);
    }
  })

  var i = 0, timeOut = 0;
  $("#volume-down")
      .on('mousedown touchstart',
          function(e) {
            e.preventDefault();
            timeOut = setInterval(function() { volume_change(-5); }, 100);
          })
      .bind('mouseup mouseleave touchend',
            function() { clearInterval(timeOut); });

  $("#volume-up")
      .on('mousedown touchstart',
          function(e) {
            e.preventDefault();
            timeOut = setInterval(function() { volume_change(5); }, 100);
          })
      .bind('mouseup mouseleave touchend',
            function() { clearInterval(timeOut); });

  $("#video-volume input").change(function() {
    selectedClient = $("#clientSelection").val();
    var value = $(this).val();
    $("#video-volume-output").html(value);
    $.ajax({
      type : "POST",
      url : "/volume",
      contenType : "application/json",
      data : JSON.stringify({"value" : value, "client" : selectedClient}),
      success : function(
          data) { $("#video-volume input[name='range']").val(value); },
      dataType : 'json'
    });
  })
})

function get_volume() {
  $("#volume-unmute").hide();
  $("#volume-mute").show();
  $.ajax({
    type : "GET",
    url : "/volume",
    success : function(data) {
      console.log(data);
      $("#video-volume input").val(data.data);
      $("#video-volume-output").html(data.data);
    },
  });
}

function volume_change(param) {

  $("#volume-unmute").hide();
  $("#volume-mute").show();
  selectedClient = $("#clientSelection").val();
  let value = $("#video-volume input[name='range']").val();
  value = parseInt(value) + param;
  $.ajax({
    type : "POST",
    url : "/volume",
    contenType : "application/json",
    data :
        JSON.stringify({"value" : value.toString(), "client" : selectedClient}),
    success : function(data) {
      $("#video-volume-output").html(value);
      $("#video-volume input[name='range']").val(value);
    },
    dataType : 'json'
  });
}
