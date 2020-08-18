var playerStatusInterval;
$(function() {
  get_video_status();
  tabbing();
  get_volume();
  init_series();
  init_movies();
  init_favourites();
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
          playerStatusInterval = setInterval(get_video_status, 500);
        }
      },
      dataType : 'json'
    });
  });
  $(document).on("click", "button[data-dismiss='modal']",
                 function() { $.modal.close(); })


  $(document).on("click", ".play-video-link", function() {
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

  $(document).on("click", ".add-movie", function() {
    value = $(this).data("id");
    path = $(this).data("path");

    $("#searchModal").attr("data-path", path);

    $('#searchModal').modal();
  });

  $(document).on("click", ".card .btn-link", function() {
    console.log("click");
    var id = $(this).attr("data-target");
    console.log(id);
    $(".card div[id='" + id + "'] .card-body").addClass("active");
    $(".card div[id!='" + id + "'] .card-body").removeClass("active");
  });

  $(document).on("click", ".add-serie", function() {
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

  $(document).on("click", ".ignore-serie", function() {
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
    //
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

  $("#video-volume input").change(function() {
    selectedClient = $("#clientSelection").val();
    var value = $(this).val();
    $("#video-volume-output").html(value);
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
    playerStatusInterval = setInterval(get_video_status, 500);
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
    clearInterval(playerStatusInterval);
    $.ajax({
      type : "GET",
      url : "/player/stop",
      success : function(data) { return data },
    });
  })

  $("#pause_button").click(function(e) {
    e.preventDefault();
    console.log("clear interval");
    clearInterval(playerStatusInterval);
    $.ajax({
      type : "GET",
      url : "/player/pause",
      success : function() { return '{"fooo":"baaa"}' },
    });
    $("#pause_button").hide();
    $("#play_button").show();
  })

  $(document).on("click", ".add-movie-information", function() {
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

  $(document).on('input', "#video-length input", function() {
    let value = $(this).val();
    $.ajax({
      type : "POST",
      url : "/player/propery",
      contenType : "application/json",
      data : JSON.stringify({"propery" : "time-pos", "value" : value}),
      success : function(data) {
        console.log(data);
        if (data.success == "success") {
          valueElement.val("");
        }
      },
      dataType : 'json'
    });
  });

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

        $('#episodeModal').modal();
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
        $('#exampleModal').modal();
      }

    });
  })
}

function init_favourites() {

  $.ajax({
    type : "GET",
    url : "/favourites",
    dataType : "html",
    success : function(data) {
      $("#favourites").append(data);
      appendSeasonDetails();
    },
  });
}

function init_movies() {

  $.ajax({
    type : "GET",
    url : "/movies",
    dataType : "html",
    success : function(data) {
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
      $("#video-volume input").val(data.data);
      $("#video-volume-output").html(data.data);
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

function get_video_status() {

  function fancyTimeFormat(duration) {
    // Hours, minutes and seconds
    var hrs = ~~(duration / 3600);
    var mins = ~~((duration % 3600) / 60);
    var secs = ~~duration % 60;
    var ret = "";
    if (hrs > 0) {
      ret += "" + hrs + ":" + (mins < 10 ? "0" : "");
    }
    ret += "" + mins + ":" + (secs < 10 ? "0" : "");
    ret += "" + secs;
    return ret;
  }

  let test = $.ajax({
                type : "GET",
                async : false,
                url : "/player/propery?target=duration",
              }).done(function(data) {
    if (data.data != undefined) {
      $("#video-length input").attr("max", data.data);
      $.ajax({
        type : "GET",
        url : "/player/propery?target=time-pos",
        success : function(data) {
          $("#video-length input").attr("value", data.data);
          $("#video-length output").text(fancyTimeFormat(data.data));
        },
      });
    }
  });
}
function cards() {
  $(".btn-link").click(function() {
    var id = $(this).attr("aria-controls");
    $(".card div[aria-labelledby='" + id + "']").addClass("active");
    $(".card div[aria-labelledby!='" + id + "']").removeClass("active");
    ;
  })
}

function tabbing() {
  $(".nav-link").click(function() {
    var id = $(this).attr("id");
    $(".tab-content div[aria-labelledby='" + id + "']").addClass("active");
    $(".tab-content div[aria-labelledby!='" + id + "']").removeClass("active");
    ;
  })
}
