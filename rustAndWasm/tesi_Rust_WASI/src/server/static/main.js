$(document).ready( function(){
    var filename = "";
    $('input:file').on("change", function() {
        filename = this.files.item(0).name;
        $('#editing').prop('disabled', false);
        $('#imagePre').prop('hidden', false);

        const [file] = this.files
        if (file) {
          imagePre.src = URL.createObjectURL(file)
        }
        var image = new FormData();
        image.append("image",this.files.item(0));
        // Send AJAX request for file upload
        $.ajax({
            url: "/handle_file_upload",
            method: "POST",
            data: image,
            contentType: false,
            processData: false,
            dataType: "html",
            success: function(response) {
                console.log("File upload submitted successfully");
            }
        });

    });

    $('#fileForm').on("submit", function(){
        
    });
});