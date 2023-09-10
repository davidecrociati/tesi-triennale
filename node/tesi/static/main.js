$(document).ready( function(){
    var filename = "";
    $('input:file').on("change", function() {
        const [file] = this.files
        if (file) {
          imagePre.src = URL.createObjectURL(file)
        }
        $('#editing').prop('disabled', false);
        $('#centralColumn').prop('hidden', false);
        $('#rightColumn').prop('hidden', true);
    });

    $('#editButton').on("click", function(){
        $('#rightColumn').prop('hidden', true);
        var image = $('#formFileLg')[0].files[0];
        
        var form = new FormData($('#fileForm')[0]);

        form.append("image", image);
        form.append("file_name", image.name)
        if($('#specchia').prop('checked')) 
         form.set('specchia',true);
        if($('#ruota').prop('checked')) 
         form.set('ruota', 90);
        else
         form.set('ruota', 0);
        if($('#bw').prop('checked')) 
         form.set('bw',true);

        $.ajax({
            url: "/upload",
            method: "POST",
            data: form,
            processData: false,
            contentType: false,
            dataType: "text",
            success: function(data) {
                $('#rightColumn').prop('hidden', false);
                $('#imagePost').attr('src', data);
            },
            error: function(jqxhr, textStatus, errorThrown){
                alert("textStatus: "+textStatus+", error: "+errorThrown);
            } 
        });
    });
});