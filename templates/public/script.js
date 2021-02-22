
var playerStatusInterval;

$(function() {
    init_series();
    init_movies();
    init_favourites();
    triggerStartVideo();
    handlePlayerTrigger();
    attach_scan();
    search_movie_term();

    $("#video-length input").click(function () {
        propertyCall("time-pos", $(this).val().toString(),function (data) {
            get_video_status() 
        })
    })

    $(document).on("click", ".mediathekviewweb-search", function() {
        let search_term = $(".mediathekviewweb-search-input").val();
        mediathekview_search(search_term, 0) ;
    })

    $(document).on("click", ".add-serie", function() {
        value = $(this).data("id");
        path = $(this).data("path");

        addEntity("serie", function(data) {
            console.log(data);
            _this.parents(".card").hide();
            if (data.success == "success") {
                valueElement.val("");
            }
        })
    });
    $("#streams li a").click(function(e) {
        e.preventDefault();
        value = $(this).data("target");

        postPlayerCommand("play", value, function () {});

    });

    $(document).on("click", "#mediathekviewweb-next-offset", function() {
        console.log("click");
        let search_term = $(".mediathekviewweb-search-input").val();
        offset = $(this).attr("data-offset");
        newOffset = parseInt(offset)+25;


        mediathekview_search(search_term, newOffset) ;
        $(this).attr("data-offset", newOffset);
        $("#mediathekviewweb-prev-offset").attr("data-offset", newOffset);
    })

    $(document).on("click", "#mediathekviewweb-prev-offset", function() {
        let search_term = $(".mediathekviewweb-search-input").val();
        offset = $(this).attr("data-offset");
        newOffset = parseInt(offset)-25;
        mediathekview_search(search_term, newOffset);
        console.log(offset);
        console.log(newOffset);
        $(this).attr("data-offset", newOffset);
        $("#mediathekviewweb-next-offset").attr("data-offset", newOffset);
    })

    $(document).on("click", "button[data-dismiss='modal']",
        function() { $.modal.close(); })

    $(document).on("click", ".card .btn-link", function() {
        const activeBody = document.querySelector(".card .card-body.active")

        var id = $(this).attr("data-target");
        $(".card div .card-body").removeClass("active");

        if (!activeBody || id !== activeBody.parentNode.id) {
            $(".card #" + id + " .card-body").addClass("active");
        }

        $(this).get(0).scrollIntoView()
    });

    $(document).on("click", ".play-video-link", function() {
        selectedClient = $("#clientSelection").val();
        value = $(this).data("file");

        $.ajax({
            type : "POST",
            url : "/player",
            contenType : "application/json",
            data : JSON.stringify({"value" : value, "command" : "status"}),
            success : function(data) {
                if (data.time > 0) {
                    $('#dialog-video-start span.time').text(fancyTimeFormat(data.time));
                    $('#dialog-video-start').modal();
                } else {
                    postPlayerCommand("play", value, function () {});
                }
            },
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

    $(document).on("click", ".add-movie-information", function() {
        let id = $(this).data("id");
        // because of a bug in $(elem).data(); we are using attr
        let path = $("#searchModal").attr("data-path");
        let _this = $(this);
        addEntity("movie", function(data) {
            console.log(data);
            _this.parents(".card").hide();
            if (data.success == "success") {
                valueElement.val("");
            }
        })

    })

    function addEntity(schema, cb){
        $.ajax({
            type : "POST",
            url : "/library/add",
            contenType : "application/json",
            data : JSON.stringify({
                "path" : path,
                "tmdb_id" : parseInt(id),
                "schema" : schema
            }),
            success : cb,
            dataType : 'json'
        });

    }


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
});

function mediathekview_search(search_term, offset) {

    $.ajax({
        type : "POST",
        url : "/favourites/search",
        contenType : "application/json",
        data : JSON.stringify({"search_term" : search_term, offset: offset}),
        success : function(data) {
            $("#mediathekviewweb-result").empty();
            $("#mediathekviewweb-result").append(data);
            return data;
        },
        dataType : 'html'
    });

}

function postPlayerCommand(command, value, cb){
    if (value === null || value === undefined) {
        $.ajax({
            type : "POST",
            url : "/player",
            contenType : "application/json",
            data : JSON.stringify({"command" : command}),
            success : cb,
            dataType : 'json'
        })
    } else {
        $.ajax({
            type : "POST",
            url : "/player",
            contenType : "application/json",
            data : JSON.stringify({"command" : command, value: value}),
            success : cb,
            dataType : 'json'
        })
    }
}

function handlePlayerTrigger() {
    $("#skip_forward_button").click(function() { skip_time_position(30); });
    $("#skip_back_button").click(function() { skip_time_position(-30); });

    $("#play_button").click(function(e) {
        e.preventDefault();
        $("#play_button").hide();
        $("#pause_button").show();
        clearInterval(playerStatusInterval);
        playerStatusInterval = setInterval(get_video_status, 500);
        postPlayerCommand("resume");
    })

    $("#stop_button").click(function(e) {
        e.preventDefault();
        clearInterval(playerStatusInterval);
        postPlayerCommand("stop");

    })

    $("#pause_button").click(function(e) {
        e.preventDefault();
        clearInterval(playerStatusInterval);
        postPlayerCommand("pause");
        $("#pause_button").hide();
        $("#play_button").show();
    })


}

function triggerStartVideo(){

    $("#startVideo").submit(function(e) {
        // post to /
        e.preventDefault(e);
        selectedClient = $("#clientSelection").val();
        valueElement = $(this).find("input[name='target']");
        value = $(this).find("input[name='target']").val();

        postPlayerCommand("play", null, function () {
            if (data.success == "success") {
                valueElement.val("");
                clearInterval(playerStatusInterval);
                playerStatusInterval = setInterval(get_video_status, 500);
            }
        });
    });
}

function init_favourites() {
    getHtmlResponse("/favourites", function(data) {
        $("#favourites").append(data); 
    }) 
}

function init_movies() {
    getHtmlResponse("/movies", function(data) {
        $("#movies").append(data); 
    }) 
}
// append all series
function init_series() {
    getHtmlResponse("/series", function(data) {
        $("#tv-shows").append(data); 
        appendSeasonDetails();
    }) 
}

function appendSeasonDetails() {
    $(".season-detail").click(function() {
        tmdb_id = $(this).data("serie");
        season_id = $(this).data("season");
        url = "/episodes/" + tmdb_id + "/" + season_id;
        getHtmlResponse(url, function(data) {
            console.log(data);
            $("#EpisodeBoxModalContent").empty();
            $("#EpisodeBoxModalContent").append(data);
            $('#episodeModal').modal();
        }) 
    });
}

function skip_time_position(time_in_second) {
    propertyCall("time-pos", null,function (data) {
        $("#video-length input").attr("value", data.data);
        let value = parseInt(data.data) + time_in_second;
        propertyCall("time-pos", value.toString(),function (data) {})
    })
}

function get_video_status() {
    propertyCall("duration", null, function(data){
        if (data.data != undefined) {
            $("#video-length input").attr("max", data.data);
            propertyCall("time-pos", null, function(data){
                $("#video-length input").attr("value", data.data);
                $("#video-length output").text(fancyTimeFormat(data.data));
            })
        }
    });

}

function attach_scan() {
    $("#scan").click(function() {
        getHtmlResponse("/library/scan", function(data) {
            $("#SearchBoxModalContent").empty();
            $("#SearchBoxModalContent").append(data);
            $('#exampleModal').modal();
        });
    })
}

function propertyCall(property, value, cb) {

    if (value == null) {

        $.ajax({
            type : "POST",
            url : "/player/property",
            contenType : "application/json",
            data : JSON.stringify(
                {"property" : property}),
            success : cb,
            dataType : 'json'
        });
    } else {
        $.ajax({
            type : "POST",
            url : "/player/property",
            contenType : "application/json",
            data : JSON.stringify(
                {"property" : property, "value" : value.toString()}),
            success : cb,
            dataType : 'json'
        });
    }

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

function getHtmlResponse(path, cb) {
    $.ajax({
        type : "GET",
        url : path,
        dataType : "html",
        success : cb,
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


