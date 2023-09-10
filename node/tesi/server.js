const express = require('express');
const multer  = require('multer');
const Jimp = require('jimp');
const app = express()
const port = 3000
const storage = multer.diskStorage({
  destination: 'img/uploaded/',
  filename: (req, file, cb) => {
    cb(null, file.originalname);
  }
});
const upload = multer({ storage });

app.use(express.static('./static'))
app.use(express.static('./img/modified'))


app.post('/upload', upload.single('image'), (req, res) => {
  try{
    const uploadededFilePath = 'img/uploaded/' + req.file.originalname
    const modifiedFilePath = 'img/modified/' + req.file.originalname
    let editings = {
      scala: Number(req.body.scala),
      contrasto: Number(req.body.contrasto),
      luminosita: Number(req.body.luminosita),
      ruota: Number(req.body.ruota),
      specchia: Boolean(req.body.specchia),
      bw: Boolean(req.body.bw)
    };
    console.log(editings)
    Jimp.read(uploadededFilePath, (err, img) => {
      if (err) throw err
      else{
        img.scale(editings.scala)
           .rotate(editings.ruota)
           .mirror(editings.specchia,false)
           .contrast(editings.contrasto/100)
           .brightness(editings.luminosita/100, function(){
              if(editings.bw)
                img.grayscale()
                .write(modifiedFilePath)
              else
                img.write(modifiedFilePath)
              res.status(200).send(req.file.originalname);
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