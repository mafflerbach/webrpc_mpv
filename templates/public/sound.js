$(function() {
    get_volume();

    // unmute sound
    $("#volume-unmute").click(function() {
        $("#volume-unmute").hide();
        $("#volume-mute").show();
        selectedClient = $("#clientSelection").val();
        let value = $("#video-volume input[name='range']").data("before");
        postVolume (
            {"value" : parseInt(value)},
            function(data) { 
                let value = $("#video-volume input[name='range']").val();
                $("#video-volume-output").html(value);
                $("#video-volume input[name='range']").val(value);
            });
	});

    // mute sound
    $("#volume-mute").click(function() {
        selectedClient = $("#clientSelection").val();
        $("#volume-unmute").show();
        $("#volume-mute").hide();

        postVolume (
            {"value" : 0},
            function(data) { 
                let value = $("#video-volume input[name='range']").val();
                $("#video-volume input[name='range']").data("before", value);
                $("#video-volume-output").html(value);
                $("#video-volume input[name='range']").val(0);
            });
	});

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

            postVolume (
                {"value" : parseInt(value)},
                function(data) { 
                    $("#video-volume input[name='range']").val(value);
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
            $("#video-volume input").val(data.data);
            $("#video-volume-output").html(data.data);
        }
    });
}

function postVolume(data, cb) {

    $.ajax({
        type : "POST",
        url : "/volume",
        contenType : "application/json",
        data : JSON.stringify(data),
        success : cb,
        dataType : 'json'
    });
}

function volume_change(param) {
    console.log("change volume");
    $("#volume-unmute").hide();
    $("#volume-mute").show();
    selectedClient = $("#clientSelection").val();
    let value = $("#video-volume input[name='range']").val();
    value = parseInt(value) + param;
    if (value < 0) {
        value = 0
    } else if (value > 100) {
        value = 100
    }
    postVolume (
        {"value" : value},
        function(data) { 
            $("#video-volume-output").html(value);
            $("#video-volume input[name='range']").val(value);
    });
}
