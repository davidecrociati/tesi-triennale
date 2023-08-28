$(document).ready( function(){
    var filename = "";
    $('input:file').on("change", function() {
        const [file] = this.files
        if (file) {
          imagePre.src = URL.createObjectURL(file)
        }
        $('#editing').prop('disabled', false);
        $('#centralColumn').prop('hidden', false);
    });

    $('#editButton').on("click", function(){
        var image = $('#formFileLg')[0].files[0];
        
        var form = new FormData($('#fileForm')[0]);

        form.append("image", image);
        form.append("file_name", image.name)
        if(!$('#specchia').prop('checked')) form.append("specchia", "false");
        else form.set('specchia',true);
        if(!$('#ruota').prop('checked')) form.append("ruota", "false");
        else form.set('ruota',true);
        if(!$('#bw').prop('checked')) form.append("bw", "false");
        else form.set('bw',true);

        $.ajax({
            url: "/upload",
            method: "POST",
            data: form,
            processData: false,
            contentType: false,
            dataType: "text",
            success: function(data) {
                $('#rightColumn').prop('hidden', false);
                $('#imagePost').attr('src', data)

            },
            error: function(jqxhr, textStatus, errorThrown){
                alert("textStatus: "+textStatus+", error: "+errorThrown);
            } 
        });
    });
});