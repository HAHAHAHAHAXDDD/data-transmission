window.onload = loadTasks

function loadTasks(){
    let tasks = Array.from(JSON.parse(localStorage.getItem("zadania")))

    tasks.forEach(task =>{
        var t = document.createElement("li")
        t.contentEditable = true
        var tv = document.createTextNode(task.zadanie)
        t.appendChild(tv)
        document.getElementById("tlist").appendChild(t)
        var date = document.createElement("li");
        date.contentEditable = true
        var btn = document.createElement("button")
        btn.innerHTML = "delete"
        btn.className = "material-symbols-outlined"
        btn.onclick = deleteItem
        var d = document.createTextNode(task.data);
        date.appendChild(d);
        date.appendChild(btn)
        document.getElementById("dlist").appendChild(date)
    })

}

function err(x){
    var error = document.getElementById("errMess");
    if (x){
        error.innerHTML = "Wprowadzone zadanie jest za długie!";      
    }
    error.setAttribute("style", "display:block")
}

function isDateValid(date){
    var dob = new Date(date)
    dob.setHours(0, 0, 0, 0)
    var today = new Date()
    today.setHours(0, 0, 0, 0)
    if (dob > today) {
        return true
    } 
    return false
}

function insertItem() {
    var errDate = document.getElementById("errDate")
    errDate.setAttribute("style", "display:none")
    var errMsg = document.getElementById("errMess")
    errMsg.setAttribute("style", "display:none")

    var task = document.createElement("li");
    task.contentEditable = true
    var inputValue = document.getElementById("insert").value;
    var t = document.createTextNode(inputValue);
    task.appendChild(t);
    var date = document.createElement("li");
    date.contentEditable = true
    var btn = document.createElement("button")
    btn.innerHTML = "delete"
    btn.className = "material-symbols-outlined"
    btn.onclick = deleteItem
    var inputDate = document.getElementById("date").value;
    var d = document.createTextNode(inputDate);
    date.appendChild(d);
    date.appendChild(btn)

    if (inputValue.length < 3) {
        err(0)
    }
    else if (inputValue.length > 255){
        err(1)
    }
    else if (!inputDate){
        date.removeChild(btn)
        d = document.createTextNode("--")
        date.appendChild(d)
        date.appendChild(btn)
        localStorage.setItem("zadania", JSON.stringify([...JSON.parse(localStorage.getItem("zadania") || "[]"), { zadanie: inputValue, data: "---" }]));
        document.getElementById("tlist").appendChild(task)
        document.getElementById("dlist").appendChild(date)
        document.getElementById("insert").value = ''
        document.getElementById("date").value = ''

    }
    else if (!isDateValid(inputDate)){
        var errDate = document.getElementById("errDate")
        errDate.setAttribute("style", "display:block")
    }
    else{
        localStorage.setItem("zadania", JSON.stringify([...JSON.parse(localStorage.getItem("zadania") || "[]"), { zadanie: inputValue, data: inputDate }]));
        document.getElementById("tlist").appendChild(task)
        document.getElementById("dlist").appendChild(date)
        document.getElementById("insert").value = ''
        document.getElementById("date").value = ''
    }
    
}

function deleteItem(){
    var firstList = document.getElementById('tlist')
    var secondList = document.getElementById('dlist')
    for (let i = 0; i < secondList.children.length; i++)
    {
        secondList.children[i].onclick = function(){
            removeStorageItem(i)
            firstList.removeChild(firstList.childNodes[i+1])
            secondList.removeChild(secondList.childNodes[i+1])
        }
    }
}

function removeStorageItem(index){
    let tasks = Array.from(JSON.parse(localStorage.getItem("zadania")))
    tasks.splice(index, 1)
    localStorage.setItem("zadania", JSON.stringify(tasks))
}

function searchTodo(){
    var input = document.getElementById("search")
    var filter = input.value.toUpperCase()
    var ul = document.getElementById("tlist")
    var ul2 = document.getElementById("dlist")
    var li = ul.getElementsByTagName('li')
    var li2 = ul2.getElementsByTagName('li')
    var txtValue;

    for (let i = 0; i < li.length; i++) {
        txtValue = li[i].textContent || li[i].innerText
        
            if (txtValue.toUpperCase().indexOf(filter) > -1 || filter.length < 3){
                li[i].style.display = ""
                li2[i].style.display = ""
            }     
            else{
                li[i].style.display = "none"
                li2[i].style.display = "none"
            }

    }
}