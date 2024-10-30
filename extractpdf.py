import pytesseract
from PIL import Image

# If Tesseract is not in your PATH, specify the path to the tesseract executable
# Example for Windows users: pytesseract.pytesseract.tesseract_cmd = r'C:\Program Files\Tesseract-OCR\tesseract.exe'

# Open an image file
image_path = 'diogo.png.png'
image = Image.open(image_path)

# Use pytesseract to extract text
extracted_text = pytesseract.image_to_string(image)

# Print the extracted text
print("Extracted Text:\n", extracted_text)

f = open("diogoeummerda", "w")

f.write(extracted_text)