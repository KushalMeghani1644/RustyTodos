import { invoke } from "@tauri-apps/api/core";

const descriptionInput = document.getElementById(
  "description",
) as HTMLInputElement;
const dueDateInput = document.getElementById("due_date") as HTMLInputElement;
const addButton = document.getElementById("addButton") as HTMLButtonElement;
const todosContainer = document.getElementById("todos") as HTMLDivElement;

async function loadTodos() {
  const todos: any[] = await invoke("get_todos");
  todosContainer.innerHTML = "";

  todos.forEach((todo, index) => {
    const item = document.createElement("div");
    item.className =
      "flex items-center justify-between bg-white shadow p-2 rounded mb-2";

    const left = document.createElement("div");
    left.className = "flex items-center gap-2";

    const checkbox = document.createElement("input");
    checkbox.type = "checkbox";
    checkbox.checked = todo.done;
    checkbox.onclick = async () => {
      await invoke("mark_done", { index });
      await loadTodos();
    };

    const label = document.createElement("span");
    label.innerText = `${todo.description} (Due: ${todo.due_date ?? "None"})`;

    left.appendChild(checkbox);
    left.appendChild(label);

    const deleteBtn = document.createElement("button");
    deleteBtn.innerText = "Delete";
    deleteBtn.className = "text-red-600 hover:text-red-800 px-2";
    deleteBtn.onclick = async () => {
      if (confirm("Are you sure you want to delete this todo?")) {
        await invoke("delete_todo", { index });
        await loadTodos();
      }
    };

    item.appendChild(left);
    item.appendChild(deleteBtn);
    todosContainer.appendChild(item);
  });
}

addButton.onclick = async () => {
  const description = descriptionInput.value.trim();
  const due = dueDateInput.value.trim();

  if (!description) return alert("Description is required.");

  try {
    await invoke("add_todo", {
      args: {
        description,
        due_date: due === "" ? undefined : due,
      },
    });
    descriptionInput.value = "";
    dueDateInput.value = "";
    await loadTodos();
  } catch (err) {
    alert("Failed to add todo: " + err);
  }
};

async function startup() {
  await invoke("load_tasks");
  await loadTodos();
}

startup();
