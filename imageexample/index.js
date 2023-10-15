import sharp from 'sharp';

const image_file = 'cat-551554_1280.jpg';
console.log(sharp);
await sharp(image_file).toFile('cat.jpg');
