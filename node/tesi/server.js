const express = require('express');
const multer  = require('multer');
const Jimp = require('jimp');
const app = express()
const port = 3000
const storage = multer.diskStorage({
  destination: 'img/uploaded/',
  filename: (req, file, cb) => {
    cb(null, Date.now() + file.originalname);
  }
});
const upload = multer({ storage });

var start = process.hrtime();

var elapsed_time = function(note){
    var precision = 3; // 3 decimal places
    var elapsed = process.hrtime(start)[1] / 1000000; // divide by a million to get nano to milli
    if(note != "new request")
      console.log((process.hrtime(start)[0]*1000 + elapsed).toFixed(precision)+ " ms - " + note); // print message + time
    start = process.hrtime(); // reset the timer
}

app.use(express.static('./static'))
app.use(express.static('./img/modified'))

app.post('/upload', upload.single('image'), (req, res) => {
  try{
    const uploadededFilePath = 'img/uploaded/' + req.file.filename;
    const modifiedFilePath = 'img/modified/' + req.file.filename;
    
    let editings = {
      scala: Number(req.body.scala),
      contrasto: Number(req.body.contrasto),
      luminosita: Number(req.body.luminosita),
      ruota: Number(req.body.ruota),
      specchia: Boolean(req.body.specchia),
      bw: Boolean(req.body.bw)
    };
    console.log(editings)

    elapsed_time("new request");
    Jimp.read(uploadededFilePath, (err, img) => {
      if (err) throw err
      else{
        elapsed_time("time for reading file:  " + uploadededFilePath);
        img.scale(editings.scala)
           .rotate(editings.ruota)
           .mirror(editings.specchia,false)
           .contrast(editings.contrasto/100)
           .brightness(editings.luminosita/100, function(){
              if(editings.bw)
                img.grayscale()

              elapsed_time("time for editing file:  " + modifiedFilePath);
              img.write(modifiedFilePath, function(){
                elapsed_time("time for writing file:  " + modifiedFilePath);
                res.status(200).send(req.file.filename);
              });
          });
      }
    });

  } catch (error){
    console.log(error);
    res.status(500).send('Error processing the file');
  }
})

app.listen(port, () => {
  console.log(`Server listening on port ${port}`)
})