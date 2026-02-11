const coursesFileInput = document.getElementById("coursesFile");
const roomsFileInput = document.getElementById("roomsFile");

coursesFileInput.addEventListener("change", () => onFileSelect("courses"));
roomsFileInput.addEventListener("change", () => onFileSelect("rooms"));