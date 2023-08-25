$(document).ready( function(){
    var filename = "";
    $('input:file').on("change", function() {
        filename = this.files.item(0).name;
        $('#editing').prop('disabled', false);
        $('#centralColumn').prop('hidden', false);

        const [file] = this.files
        if (file) {
          imagePre.src = URL.createObjectURL(file)
        }
        var image = new FormData();
        image.append("image",this.files.item(0));
        // Send AJAX request for file upload
        $.ajax({
            url: "upload",
            method: "POST",
            data: image,
            contentType: false,
            processData: false,
            success: function(response) {
                console.log(response);
            }
        });

    });

    $('#editButton').on("click", function(){
        console.log($("#fileForm").serialize());
        $.ajax({
            url: "/edit",
            method: "POST",
            data: $("#fileForm").serialize(),
            dataType: "html",
            success: function(data) {
                console.log("File upload submitted successfully");
            }
        });
    });
});