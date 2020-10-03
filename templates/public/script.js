var playerStatusInterval;
$(function() {
  get_video_status();
  init_series();
  init_movies();
  init_favourites();
  search_movie_term();
  get_pos_property();

  // start playlist
  $("#start-playlist").click(function() {
    $.ajax({
      type : "GET",
      url : "/player/playlist",
      success : function() { return '{"fooo":"baaa"}' },
    });
  });

  $(document).on("click", "button[data-dismiss='modal']",
                 function() { $.modal.close(); })

  $("#dialog-video-start button[data-dismiss='modal'").click(function() {
    $.ajax({
      type : "POST",
      url : "/player/status",
      contenType : "application/json",
      data : JSON.stringify({"path" : value, "client" : selectedClient}),
      success : function(data) {
        console.log(data);
        let time = data.time;
        $.ajax({
          type : "POST",
          url : "/player",
          contenType : "application/json",
          data : JSON.stringify({"target" : value, "client" : selectedClient}),
          success : function(data) {
            $.ajax({
              type : "POST",
              url : "/player/propery",
              contenType : "application/json",
              data : JSON.stringify({"propery" : "time-pos", "value" : "0"}),
              success : function(data) {},
              dataType : 'json'
            });
          },
          dataType : 'json'
        });
      },
      dataType : 'json'
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
  $("#dialog-video-start button[data-confirm='modal'").click(function() {
    selectedClient = $("#clientSelection").val();
    $.ajax({
      type : "POST",
      url : "/player/status",
      contenType : "application/json",
      data : JSON.stringify({"path" : value, "client" : selectedClient}),
      success : function(data) {
        console.log(data);
        let time = data.time;
        $.ajax({
          type : "POST",
          url : "/player",
          contenType : "application/json",
          data : JSON.stringify({"target" : value, "client" : selectedClient}),
          success : function(data) {
            setTimeout(() => {
              $.ajax({
                type : "POST",
                url : "/player/propery",
                contenType : "application/json",
                data : JSON.stringify(
                    {"propery" : "time-pos", "value" : time.toString()}),
                success : function(data) {},
                dataType : 'json'
              });
            }, 200);
          },
          dataType : 'json'
        });
      },
      dataType : 'json'
    });
  });

  $(document).on("click", ".play-video-link", function() {
    selectedClient = $("#clientSelection").val();
    value = $(this).data("file");

    $.ajax({
      type : "POST",
      url : "/player/status",
      contenType : "application/json",
      data : JSON.stringify({"path" : value, "client" : selectedClient}),
      success : function(data) {
        if (data.time > 0) {
          $('#dialog-video-start span.time').text(fancyTimeFormat(data.time));
          $('#dialog-video-start').modal();
        } else {
          $.ajax({
            type : "POST",
            url : "/player",
            contenType : "application/json",
            data :
                JSON.stringify({"target" : value, "client" : selectedClient}),
            success : function(data) { console.log(data); },
            dataType : 'json'
          });
        }
      },
      dataType : 'json'
    });
  });

  // opens movie search for adding a movie with details
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

  // add movie with movie details
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

  // handle arccordeons in episode list
  $(document).on("click", ".card .btn-link", function() {
    var id = $(this).attr("data-target");
    $(".card div[id!='" + id + "'] .card-body").removeClass("active");
    $(".card div[id='" + id + "'] .card-body").addClass("active");
  });

  // add serie
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

  // add to ignore list
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

  // Open live streams in mediathekview
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
  // add file to playlist
  $("#addPlaylist").submit(function(e) {
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

  // search for videos in mediathekview
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

  // sync video position with bar
  $("#play_button").click(function(e) {
    e.preventDefault();
    $("#play_button").hide();
    $("#pause_button").show();
    playerStatusInterval = setInterval(get_video_status, 500);
    $.ajax({
      type : "GET",
      url : "/player/resume",
      success : function() { return '{"fooo":"baaa"}' },
    });
  })

  // skip 1 min forward and backward
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

  // set time position in video based on bar
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
  // attach scan event
  attach_scan();
});

// search for movie detail while adding a video
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
// start scan of the library for new videos and series
function attach_scan() {
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

// append all favourites
function init_favourites() {
  $.ajax({
    type : "GET",
    url : "/favourites",
    dataType : "html",
    success : function(data) { $("#favourites").append(data); },
  });
}
// append all videos
function init_movies() {
  $.ajax({
    type : "GET",
    url : "/movies",
    dataType : "html",
    success : function(data) { $("#movies").append(data); },
  });
}
// append all series
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

// append episode Details when click on season button
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

function get_video_status() {

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
