let post_crime = function(lat, long, desc) {
    let xhr = new XMLHttpRequest();
    xhr.open("POST", "/report");

    xhr.setRequestHeader("Accept", "application/json");
    xhr.setRequestHeader("Content-Type", "application/json");

    xhr.onload = () => console.log(xhr.responseText);

    let data = {
        "lat": lat,
        "long": long,
        "desc": desc,
        "loc": ""
    };

    xhr.send(JSON.stringify(data));
}

let register_user = function(num, lat, long) {
    let xhr = new XMLHttpRequest();
    xhr.open("POST", "/register");

    xhr.setRequestHeader("Accept", "application/json");
    xhr.setRequestHeader("Content-Type", "application/json");

    xhr.onload = () => console.log(xhr.responseText);

    let data = {
        "num": num,
        "lat": lat,
        "long": long,
    };

    xhr.send(JSON.stringify(data));
}

let displayForm = function() {
    canReport = false;
    document.getElementById("myModal").style.display = "block";
}

let closeForm = function() {
    let old_marker = document.getElementById("user marker");

    if (old_marker != null) {
        old_marker.remove();
    }

    document.getElementById("myModal").style.display = "none";
}

let submitForm = function() {
    post_crime(lat, long, document.getElementById("desc").value);
    closeForm();
}