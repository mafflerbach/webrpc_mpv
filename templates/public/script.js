$(function() {
    get_volume();

    $("#startVideo").submit(function(e) {
        // post to /
        e.preventDefault(e);
        selectedClient = $("#clientSelection").val();
        valueElement = $(this).find("input[name='target']");
        value = $(this).find("input[name='target']").val();
        $.ajax({
            type: "POST",
            url: "/player",
            contenType: "application/json",
            data: JSON.stringify({
                "target": value,
                "client": selectedClient
            }),
            success: function(data) {
                console.log(data);
                if (data.success == "success") {
                    valueElement.val("");
                }
            },
            dataType: 'json'
        });

    });


    $("#add-serie").click(function() {

        $.ajax({
            type: "POST",
            url: "/serie/add",
            contenType: "application/json",
            data: JSON.stringify({
                "target": value,
                "client": selectedClient
            }),
            success: function(data) {
                console.log(data);
                if (data.success == "success") {
                    valueElement.val("");
                }
            },
            dataType: 'json'
        });

    });
    $("#ignore-serie").click(function() {

        $.ajax({
            type: "POST",
            url: "/serie/ignore",
            contenType: "application/json",
            data: JSON.stringify({
                "target": value,
                "client": selectedClient
            }),
            success: function(data) {
                console.log(data);
                if (data.success == "success") {
                    valueElement.val("");
                }
            },
            dataType: 'json'
        });

    });


    $("#addPlaylist").submit(function(e) {
        //  post to /add

        e.preventDefault(e);

        selectedClient = $("#clientSelection").val();

        value = $(this).find("input[name='target']").val()
        $.ajax({
            type: "POST",
            url: "/player/add",
            contenType: "application/json",
            data: JSON.stringify({
                "value": value,
                "client": selectedClient
            }),
            success: function() {
                return '{"fooo":"baaa"}'
            },
            dataType: 'json'
        });
    });

    $("#range input").change(function() {
        var value = $(this).val();
        $.ajax({
            type: "POST",
            url: "/volume",
            contenType: "application/json",
            data: JSON.stringify({
                "value": value
            }),
            success: function(data) {
                return data;
            },
            dataType: 'json'
        });
    })

    $("#play_button").click(function(e) {
        e.preventDefault();
        $.ajax({
            type: "GET",
            url: "/player/resume",
            success: function() {
                return '{"fooo":"baaa"}'
            },
        });
        $("#play_button").hide();
        $("#pause_button").show();
    })
    
    $("#stop_button").click(function(e) {
        e.preventDefault();
        $.ajax({
            type: "GET",
            url: "/player/stop",
            success: function(data) {
                return data
            },
        });
    })

    $("#pause_button").click(function(e) {
        e.preventDefault();
        $.ajax({
            type: "GET",
            url: "/player/pause",
            success: function() {
                return '{"fooo":"baaa"}'
            },
        });
        $("#pause_button").hide();
        $("#play_button").show();
    })

    $("#scan").click(function() {
        $.ajax({
            type: "GET",
            url: "/scan",
            success: function(data) {

                console.log(data.results);

                for (let i = 0; i < data.length; i++) {
                    let elem = data[i];

                    let clone = $("#searchResultCloneable").clone();
                    clone.removeAttr("id");
                    let bgImage = "https://image.tmdb.org/t/p/w500" + elem.poster_path;
                    clone.find(".cardContent").css("background-image", "url(" + bgImage + " )");
                    clone.find(".card-title").append(elem.name);
                    clone.find(".card-text").append(elem.overview);
                    clone.find("#ignore-serie").attr("id", elem.id);
                    clone.find("#add-serie").attr("id", elem.id);

                    clone.appendTo("#SearchBoxModalContent");
                }
            },

        });
    });

});

function get_volume() {
    $.ajax({
        type: "GET",
        url: "/volume",
        success: function(data) {
            console.log(data);
            $("#range input").val(data.data);
        },
    });

}
