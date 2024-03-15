/// Get the HTML content to render the home/listing page.
///
/// # See Also
///
/// - This page is served as a response for the `/home` entry point.
///
/// # Returns
///
/// A `String` version of the HTML, CSS and JS content.
pub fn get_content() -> String {
    r###"<!DOCTYPE html>
<!--suppress JSUnresolvedLibraryURL -->
<html lang="en">
<head>
    <meta http-equiv="Content-Type" content="text/html; charset=utf-8">
    <title>RuStream - Self-hosted Streaming Engine - v{{ version }}</title>
    <meta property="og:type" content="MediaStreaming">
    <meta name="keywords" content="Rust, streaming, actix, JavaScript, HTML, CSS">
    <meta name="author" content="Vignesh Rao">
    <meta content="width=device-width, initial-scale=1" name="viewport">
    <!-- Favicon.ico and Apple Touch Icon -->
    <link rel="icon" href="https://thevickypedia.github.io/open-source/images/logo/actix.ico">
    <link rel="apple-touch-icon" href="https://thevickypedia.github.io/open-source/images/logo/actix.png">
    <!-- Font Awesome icons -->
    <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/6.5.1/css/fontawesome.min.css">
    <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/6.5.1/css/solid.css">
    <!-- CSS and JS for night mode -->
    <script src="https://cdnjs.cloudflare.com/ajax/libs/jquery/2.2.2/jquery.min.js"></script>
    <script type="text/javascript" src="https://thevickypedia.github.io/open-source/nightmode/night.js" defer></script>
    <link rel="stylesheet" type="text/css" href="https://thevickypedia.github.io/open-source/nightmode/night.css">
    <!-- Button CSS -->
    <style>
        /* Google fonts with a backup alternative */
        @import url('https://fonts.googleapis.com/css2?family=Ubuntu:wght@400;500;700&display=swap');
        * {
            font-family: 'Ubuntu', 'PT Serif', sans-serif;
        }
        body {
            margin-left: 1%;  /* 1% away from left corner */
            padding: 0.5%  /* 0.5% away from any surrounding elements */
        }
        small {
            font-size: 16px;
        }
        .upload {
            position: absolute;
            top: 3.8%;
            right: 313px;
            border: none;
            padding: 10px 14px;
            font-size: 16px;
            cursor: pointer;
        }
        .home {
            position: absolute;
            top: 3.8%;
            right: 217px;
            border: none;
            padding: 10px 14px;
            font-size: 16px;
            cursor: pointer;
        }
        .back {
            position: absolute;
            top: 3.8%;
            right: 132px;
            border: none;
            padding: 10px 14px;
            font-size: 16px;
            cursor: pointer;
        }
    </style>
    <style>
        .dropbtn {
            position: absolute;
            top: 3.8%;
            right: 30px;
            padding: 10px 24px;
            font-size: 16px;
            border: none;
            cursor: pointer;
        }
        .dropdown {
            position: absolute;
            top: 3.8%;
            right: 30px;
            padding: 10px 24px;
            display: inline-block;
        }
        .dropdown-content {
            display: none;
            position: absolute;
            top: 40px;  /* Distance from the user icon button */
            right: 30px;
            width: 160px;
            min-width: auto;
            box-shadow: 0 8px 16px 0 rgba(0,0,0,0.2);  /* Basically, black with 20% opacity */
            z-index: 1;
        }
        .dropdown-content a {
            padding: 12px 16px;
            text-decoration: none;
            display: block;
        }
        .dropdown:hover .dropdown-content {display: block;}
    </style>
    <!-- Title list CSS -->
    <style>
        a:hover, a:active { font-size: 120%; opacity: 0.7; }
        a:link { color: blue; }
        a:visited { color: blue; }
        ol {
            list-style: none;
            counter-reset: list-counter;
        }
        li {
            margin: 1rem;
            list-style-type: none; /* Hide default marker */
        }
        li::before {
            background: #4169E1;
            width: 2rem;
            height: 2rem;
            border-radius: 50%;
            display: inline-block;
            line-height: 2rem;
            color: white;
            text-align: center;
            margin-right: 0.5rem;
        }
    </style>
    <style>
        /* Style for context menu */
        .context-menu {
            position: absolute;
            background-color: #fff;
            border: 1px solid #ccc;
            padding: 5px 0;
            box-shadow: 2px 2px 5px rgba(0, 0, 0, 0.2);
        }
        .context-menu-item {
            padding: 5px 10px;
            cursor: pointer;
            background-color: #fff !important; /* White background */
            color: #000 !important; /* Black font */
        }
        .context-menu-item:hover {
            background-color: #000 !important; /* White background */
            color: #fff !important; /* Black font */
        }
    </style>
</head>
<noscript>
    <style>
        body {
            width: 100%;
            height: 100%;
            overflow: hidden;
        }
    </style>
    <div style="position: fixed; text-align:center; height: 100%; width: 100%; background-color: #151515;">
        <h2 style="margin-top:5%">This page requires JavaScript
            to be enabled.
            <br><br>
            Please refer <a href="https://www.enable-javascript.com/">enable-javascript</a> for how to.
        </h2>
        <form>
            <button type="submit" onClick="<meta httpEquiv='refresh' content='0'>">RETRY</button>
        </form>
    </div>
</noscript>
<body translate="no">
    <div class="toggler fa fa-moon-o"></div>
    <button class="upload" onclick="upload()"><i class="fa-solid fa-cloud-arrow-up"></i> Upload</button>
    <button class="home" onclick="goHome()"><i class="fa fa-home"></i> Home</button>
    <button class="back" onclick="goBack()"><i class="fa fa-backward"></i> Back</button>
    <div class="dropdown">
        <button class="dropbtn"><i class="fa fa-user"></i></button>
        <div class="dropdown-content">
            <a onclick="goProfile()" style="cursor: pointer;"><i class="fa-solid fa-user-lock"></i> {{ user }}</a>
            <a onclick="logOut()" style="cursor: pointer"><i class="fa fa-sign-out"></i> logout</a>
        </div>
    </div>
    <br><br><br><br>
    <!-- Context menu template (hidden by default) -->
    <div id="contextMenu" class="context-menu" style="display: none;">
        <div class="context-menu-item" onclick="deleteItem(currentPath)">Delete</div>
      <!-- <div class="context-menu-item" onclick="renameItem(currentPath)">Rename</div> -->
    </div>
    {% if custom_title %}
        <h1>{{ custom_title }}</h1>
    {% else %}
        <h1>Welcome to RuStream <small>v{{ version }}</small></h1>
    {% endif %}
    <hr>
    {% if dir_name or files or directories or secured_directories %}
        <!-- Display directory name if within subdir -->
        {% if dir_name %}
            <h3>{{ dir_name }}</h3>
        {% endif %}
        <!-- Display number of files and list the files -->
        {% if files %}
            <h3>Files {{ files|length }}</h3>
            {% for file in files %}
                {% if secure_path == 'true' %}
                    <li><i class="{{ file.font }}"></i>&nbsp;&nbsp;<a oncontextmenu="showContextMenu(event, '{{ file.path }}')" href="{{ file.path }}">{{ file.name }}</a></li>
                {% else %}
                    <li><i class="{{ file.font }}"></i>&nbsp;&nbsp;<a href="{{ file.path }}">{{ file.name }}</a></li>
                {% endif %}
            {% endfor %}
        {% endif %}
        <!-- Display number of directories and list the directories -->
        {% if directories %}
            <h3>Directories {{ directories|length }}</h3>
            {% for directory in directories %}
                <li><i class="{{ directory.font }}"></i>&nbsp;&nbsp;<a href="{{ directory.path }}">{{ directory.name }}</a></li>
            {% endfor %}
        {% endif %}
        {% if secured_directories %}
            <h3>Secured Directory</h3>
            {% for directory in secured_directories %}
                <li><i class="{{ directory.font }}"></i>&nbsp;&nbsp;<a oncontextmenu="showContextMenu(event, '{{ directory.path }}')" href="{{ directory.path }}">{{ directory.name }}</a></li>
            {% endfor %}
        {% endif %}
    {% else %}
        <h3 style="text-align: center">No content was rendered by the server</h3>
    {% endif %}
    <hr>
    <script>
        function goHome() {
            window.location.href = "/home";
        }
        function goProfile() {
            window.location.href = '/profile';
        }
        function logOut() {
            window.location.href = "/logout";
        }
        function upload() {
            window.location.href = "/upload";
        }
        function goBack() {
            window.history.back();
        }
    </script>
    <script>
        var contextMenu = document.getElementById('contextMenu');

        // Function to show context menu
        function showContextMenu(event, path) {
            event.preventDefault();

            // Set the global variable to the current file path
            currentPath = path;

            // Position the context menu beneath the clicked icon
            contextMenu.style.display = 'block';
            contextMenu.style.left = event.clientX + 'px';
            contextMenu.style.top = event.clientY + 'px';
        }

        function editAction(action, trueURL, relativePath) {
            let http = new XMLHttpRequest();
            http.open('POST', window.location.origin + `/edit`, true);  // asynchronous session
            http.setRequestHeader('Content-Type', 'application/json'); // Set content type to JSON
            http.setRequestHeader('edit-action', action);
            let data = {
                url_locator: trueURL,
                path_locator: relativePath
            };
            let jsonData = JSON.stringify(data);
            http.onreadystatechange = function() {
                if (http.readyState === XMLHttpRequest.DONE) {
                    if (http.status === 200) {
                        window.location.reload();
                    } else {
                        console.error('Error:', http.status);
                    }
                }
            };
            http.send(jsonData);
        }

        function getConfirmation(fileName, action) {
            let confirmation = confirm(`Are you sure you want to ${action}?\n\n'${fileName}'`);
            if (!confirmation) {
                contextMenu.style.display = 'none';
                return false;
            }
            return true;
        }

        function extractFileName(path) {
            // Find the last occurrence of either '/' or '\'
            const lastIndex = Math.max(path.lastIndexOf('/'), path.lastIndexOf('\\'));

            // Extract the filename using substring
            return path.substring(lastIndex + 1);
        }

        // Function to handle delete action
        function deleteItem(relativePath) {
            contextMenu.style.display = 'none';

            let fileName = extractFileName(relativePath);
            let pass = getConfirmation(fileName, 'delete');
            if (!pass) {
                return;
            }
            let trueURL = window.location.href + '/' + fileName;
            editAction("delete", trueURL, relativePath);
        }

        // Function to handle rename action
        function renameItem(path) {
            contextMenu.style.display = 'none';
            alert(`Rename of ${path} is not enabled yet!!`);
        }

        // Hide context menu when clicking outside
        document.addEventListener('click', function(event) {
            if (event.target !== contextMenu && !contextMenu.contains(event.target)) {
                contextMenu.style.display = 'none';
            }
        });
        </script>
</body>
</html>
"###.to_string()
}
