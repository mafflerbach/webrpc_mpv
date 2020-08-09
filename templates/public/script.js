$(function() {
  get_volume();
  init_series();
  $("#startVideo").submit(function(e) {
    // post to /
    e.preventDefault(e);
    selectedClient = $("#clientSelection").val();
    valueElement = $(this).find("input[name='target']");
    value = $(this).find("input[name='target']").val();
    $.ajax({
      type : "POST",
      url : "/player",
      contenType : "application/json",
      data : JSON.stringify({"target" : value, "client" : selectedClient}),
      success : function(data) {
        console.log(data);
        if (data.success == "success") {
          valueElement.val("");
        }
      },
      dataType : 'json'
    });
  });


  $("#episodeModal").on("click", ".play-video-link", function() {
    

    selectedClient = $("#clientSelection").val();
    value = $(this).data("file");
    $.ajax({
      type : "POST",
      url : "/player",
      contenType : "application/json",
      data : JSON.stringify({"target" : value, "client" : selectedClient}),
      success : function(data) {
        console.log(data);
      },
      dataType : 'json'
    });

});
  $("#searchResult").on("click", ".add-serie", function() {
    value = $(this).data("id");
    path = $(this).data("path");
    $.ajax({
      type : "POST",
      url : "/library/add",
      contenType : "application/json",
      data : JSON.stringify({
        "path" : path,
        "tmdb_id" : parseInt(value),
      }),
      success : function(data) {
        console.log(data);
        if (data.success == "success") {
          valueElement.val("");
        }
      },
      dataType : 'json'
    });
  });

  $("#searchResult").on("click", ".ignore-serie", function() {
    value = $(this).data("id");
    path = $(this).data("path");
    $.ajax({
      type : "POST",
      url : "/library/ignore",
      contenType : "application/json",
      data : JSON.stringify({
        "path" : "",
        "tmdb_id" : parseInt(value),
      }),
      success : function(data) {
        console.log(data);
        if (data.success == "success") {
          valueElement.val("");
        }
      },
      dataType : 'json'
    });
  });

  $("#addPlaylist").submit(function(e) {
    //  post to /add

    e.preventDefault(e);

    selectedClient = $("#clientSelection").val();

    value = $(this).find("input[name='target']").val()
    $.ajax({
      type : "POST",
      url : "/player/add",
      contenType : "application/json",
      data : JSON.stringify({"value" : value, "client" : selectedClient}),
      success : function() { return '{"fooo":"baaa"}' },
      dataType : 'json'
    });
  });

  $("#range input").change(function() {
    var value = $(this).val();
    $.ajax({
      type : "POST",
      url : "/volume",
      contenType : "application/json",
      data : JSON.stringify({"value" : value}),
      success : function(data) { return data; },
      dataType : 'json'
    });
  })

  $("#play_button").click(function(e) {
    e.preventDefault();
    $.ajax({
      type : "GET",
      url : "/player/resume",
      success : function() { return '{"fooo":"baaa"}' },
    });
    $("#play_button").hide();
    $("#pause_button").show();
  })

  $("#stop_button").click(function(e) {
    e.preventDefault();
    $.ajax({
      type : "GET",
      url : "/player/stop",
      success : function(data) { return data },
    });
  })

  $("#pause_button").click(function(e) {
    e.preventDefault();
    $.ajax({
      type : "GET",
      url : "/player/pause",
      success : function() { return '{"fooo":"baaa"}' },
    });
    $("#pause_button").hide();
    $("#play_button").show();
  })

  scan();
});

function appendSeasonDetails() {

  $(".season-detail").click(function() {
    tmdb_id = $(this).data("serie");
    season_id = $(this).data("season");
    url = "/episodes/" + tmdb_id + "/" + season_id;
    $.ajax({
      type : "GET",
      url : url,
      dataType : "html",
      success : function(data) {
        console.log(data);
        $("#EpisodeBoxModalContent").empty();
        $("#EpisodeBoxModalContent").append(data);
$('#episodeModal').modal('show')

      },
    });
  });
}

function scan() {
  $("#scan").click(function() {
    $.ajax({
      type : "GET",
      url : "/library/scan",
      success : function(data) {

        for (let i = 0; i < data.length; i++) {
          let elem = data[i];

          let clone = $("#searchResultCloneable").clone();
          clone.removeAttr("id");
          let bgImage = "https://image.tmdb.org/t/p/w500" + elem.poster_path;
          clone.find(".cardContent")
              .css("background-image", "url(" + bgImage + " )");
          clone.find(".card-title").append(elem.name);
          clone.find(".card-text").append(elem.overview);
          clone.find(".ignore-serie").attr("data-id", elem.id);
          clone.find(".add-serie").attr("data-id", elem.id);
          clone.find(".add-serie").attr("data-path", elem.file_path);

          clone.appendTo("#SearchBoxModalContent");
        }
      }

    });
  })
}

function init_series() {

  $.ajax({
    type : "GET",
    url : "/series",
    dataType : "html",
    success : function(data) {
      console.log(data);
      $("#localvid").append(data);
      appendSeasonDetails();
    },
  });
}

function get_volume() {
  $.ajax({
    type : "GET",
    url : "/volume",
    success : function(data) {
      console.log(data);
      $("#range input").val(data.data);
    },
  });
}
