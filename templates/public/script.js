
$(function() {
    get_volume();

    $("#startVideo").submit(function(e){
        // post to /
        e.preventDefault(e);
        selectedClient = $( "#clientSelection" ).val();
        valueElement = $(this).find("input[name='target']");
        value = $(this).find("input[name='target']").val();
        $.ajax({
            type: "POST",
            url: "/",
            contenType: "application/json",
            data: JSON.stringify({ "target":  value, "client": selectedclient}),
            success: function (data) {
                console.log(data);
                if (data.success == "success") {
                    valueElement.val("");
                }
            },
            dataType: 'json'
        });

    });
    $("#addPlaylist").submit(function(e){
        //  post to /add

        e.preventDefault(e);
        
        selectedClient = $( "#clientSelection" ).val();
        value = $(this).find("input[name='target']").val()
        $.ajax({
            type: "POST",
            url: "/add",
            contenType: "application/json",
            data: JSON.stringify({ "value":  value, "client": selectedclient}),
            success: function () {
                return '{"fooo":"baaa"}'
            },
            dataType: 'json'
        });
    });

    $("#range input").change(function(){
        var value = $(this).val();
        $.ajax({
            type: "POST",
            url: "/volume",
            contenType: "application/json",
            data: JSON.stringify({ "value": value }),
            success: function () {
                return '{"fooo":"baaa"}'
            },
            dataType: 'json'
        });
    })



    $("#play_button").click(function (e) {
        e.preventDefault(); 
        $.ajax({
            type: "GET",
            url: "/resume",
            success: function () {
                return '{"fooo":"baaa"}'
            },
        });
        $("#play_button").hide();
        $("#pause_button").show();
    })

    $("#pause_button").click(function (e) {
        e.preventDefault(); 
        $.ajax({
            type: "GET",
            url: "/pause",
            success: function () {
                return '{"fooo":"baaa"}'
            },
        });
        $("#pause_button").hide();
        $("#play_button").show();
    })

});

function get_volume() {
    $.ajax({
        type: "GET",
        url: "/volume",
        success: function (data) {
            console.log(data);
            $("#range input").val(data.data);
        },
    });

}
