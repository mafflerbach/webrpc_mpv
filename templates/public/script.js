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
    let file_name_split = path.split("/");
    file = file_name_split[file_name_split.length - 1];

    file_name = file.split(".");
    $(".movie-term").val(file_name[0]);
    $('#searchModal').modal();
  });

  $(document).on("click", ".card .btn-link", function() {
    console.log("click");
    var id = $(this).attr("data-target");
    console.log(id);
    $(".card div[id!='" + id + "'] .card-body").removeClass("active");
    $(".card div[id='" + id + "'] .card-body").addClass("active");
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

  $(document).on("click",
                 ".ignore-serie, .ignore-movie, .ignore-movie-information",
                 function() {
                   value = $(this).data("id");
                   path = $(this).data("path");
                   let _this = $(this);
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
                       _this.parents(".card").hide();
                       if (data.success == "success") {
                         valueElement.val("");
                       }
                     },
                     dataType : 'json'
                   });
                 });

  $("#streams li a").click(function(e) {
    e.preventDefault();

    selectedClient = $("#clientSelection").val();
    value = $(this).data("target");
    $.ajax({
      type : "GET",
      url : "/player/play",
      contenType : "application/json",
      data : {"target" : value, "client" : selectedClient},
      success : function(data) { console.log(data); },
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
    console.log("Key down: " + e.code)
    if (e.code == "VolumeDown") {
      volume_change(-5);
    }
    else if (e.code == "VolumeUp") {
      volume_change(5);
    }
  })

  var i = 0, timeOut = 0;
  $("#volume-down")
      .on('mousedown touchstart',
          function(
              e) { timeOut = setInterval(function() { volume_change(-5); }, 100); })
      .bind('mouseup mouseleave touchend',
            function() { clearInterval(timeOut); });

  $("#volume-up")
      .on('mousedown touchstart',
          function(
              e) { timeOut = setInterval(function() { volume_change(5); }, 100); })
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

  $(document).on("click", ".mediathekviewweb-search", function() {
    let search_term = $(".mediathekviewweb-search-input").val();

    $.ajax({
      type : "POST",
      url : "/favourites/search",
      contenType : "application/json",
      data : JSON.stringify({"search_term" : search_term}),
      success : function(data) {
        $("#mediathekviewweb-result").empty();
        $("#mediathekviewweb-result").append(data);
        return data;
      },
      dataType : 'html'
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

  $("#skip_forward_button").click(function() { skip_time_position(60); });
  $("#skip_back_button").click(function() { skip_time_position(-60); });

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
    // because of a bug in $(elem).data(); we are using attr
    let path = $("#searchModal").attr("data-path");
    let _this = $(this);
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
        _this.parents(".card").hide();
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

  $(document).on("click", ".search-movie-term", function() {
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

function skip_time_position(time_in_second) {

  $.ajax({
    type : "GET",
    url : "/player/propery?target=time-pos",
    success : function(data) {
      $("#video-length input").attr("value", data.data);

      let value = parseInt(data.data) + time_in_second;

      $.ajax({
        type : "POST",
        url : "/player/propery",
        contenType : "application/json",
        data : JSON.stringify(
            {"propery" : "time-pos", "value" : value.toString()}),
        success : function(data) { get_video_status() },
        dataType : 'json'
      });
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

