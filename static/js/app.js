const coursesFileInput = document.getElementById("coursesFile");
const roomsFileInput = document.getElementById("roomsFile");

coursesFileInput.addEventListener("change", () => onFileSelect("courses"));
roomsFileInput.addEventListener("change", () => onFileSelect("rooms"));

function onFileSelect(type) {
    const input = document.getElementById(`${type}File`);
    const file = input.files[0];

    if (!file) {
        return;
    }

    document.getElementById(`${type}FileName`).textContent = file.name;
    // Generate button only available when both inputs have "has-file"
    document.getElementById(`${type}Card`).classList.add("has-file");

    const reader = new FileReader();
    reader.onload = (e) => {
        try {
            const json = JSON.parse(e.target.result);

            if (type === "courses") {
                coursesData = json;
            } else {
                roomsData = json;
            }

            const count = json[type]?.length ?? 0;
        } catch (err) {
           
        }
    };
    reader.readAsText(file);
}