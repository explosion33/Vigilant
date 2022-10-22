let post_crime = function(lat, long, desc) {
    let xhr = new XMLHttpRequest();
    xhr.open("POST", "/report");

    xhr.setRequestHeader("Accept", "application/json");
    xhr.setRequestHeader("Content-Type", "application/json");

    xhr.onload = () => console.log(xhr.responseText);

    let data = {
        "lat": lat,
        "long": long,
        "desc": desc
    };

    xhr.send(JSON.stringify(data));
}