$(function() {
  get_volume();
  init_series();
  init_movies();
  search_movie_term();
  get_pos_property();
  $("#start-playlist").click(function() {
    $.ajax({
      type : "GET",
      url : "/player/playlist",
      success : function() { return '{"fooo":"baaa"}' },
    });
  });

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

  $("#episodeModal, #movies").on("click", ".play-video-link", function() {
    selectedClient = $("#clientSelection").val();
    value = $(this).data("file");
    $.ajax({
      type : "POST",
      url : "/player",
      contenType : "application/json",
      data : JSON.stringify({"target" : value, "client" : selectedClient}),
      success : function(data) { console.log(data); },
      dataType : 'json'
    });
  });

  $("#searchResult").on("click", ".add-movie", function() {
    value = $(this).data("id");
    path = $(this).data("path");

    $("#searchModal").attr("data-path", path);
    var searchModal = new bootstrap.Modal(
        document.getElementById('searchModal'), {keyboard : false})
    searchModal.show();
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
    selectedClient = $("#clientSelection").val();
    var value = $(this).val();
    $.ajax({
      type : "POST",
      url : "/volume",
      contenType : "application/json",
      data : JSON.stringify({"value" : value, "client" : selectedClient}),
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

  $("#searchMovieContainer").on("click", ".add-movie-information", function() {
    let id = $(this).data("id");
    let path = $("#searchModal").data("path");

    $.ajax({
      type : "POST",
      url : "/library/add-movie",
      contenType : "application/json",
      data : JSON.stringify({
        "path" : path,
        "tmdb_id" : parseInt(id),
      }),
      success : function(data) {
        console.log(data);
        if (data.success == "success") {
          valueElement.val("");
        }
      },
      dataType : 'json'
    });
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

function search_movie_term() {

  $("#searchMovieContainer").on("click", ".search-movie-term", function() {
    value = $(".movie-term").val();
    $.ajax({
      type : "POST",
      url : "/movies/search-movie",
      contenType : "application/json",
      data : JSON.stringify({"term" : value}),
      success : function(data) {
        $("#movie-search-result-box").empty();
        $("#movie-search-result-box").append(data);
      },
      dataType : 'html'
    });
  });
}

function scan() {
  $("#scan").click(function() {
    $.ajax({
      type : "GET",
      url : "/library/scan",
      dataType : "html",
      success : function(data) {
        $("#SearchBoxModalContent").empty();
        $("#SearchBoxModalContent").append(data);
        var myModal = new bootstrap.Modal(
            document.getElementById('exampleModal'), {keyboard : false})
        myModal.show();
      }

    });
  })
}

function init_movies() {

  $.ajax({
    type : "GET",
    url : "/movies",
    dataType : "html",
    success : function(data) {
      console.log(data);
      $("#movies").append(data);
      appendSeasonDetails();
    },
  });
}

function init_series() {

  $.ajax({
    type : "GET",
    url : "/series",
    dataType : "html",
    success : function(data) {
      console.log(data);
      $("#tv-shows").append(data);
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
function get_pos_property() {
  $.ajax({
    type : "GET",
    url : "/player/propery?target=time-pos",
    success : function(data) {
      console.log(data.data);
      $("#play_button").hide();
      $("#pause_button").show();
      if (data.data == undefined) {

        $("#play_button").show();
        $("#pause_button").hide();
      }
    },
  });
}
