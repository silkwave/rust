import { invoke } from "@tauri-apps/api/core";

interface Todo {
  id: number;
  description: string;
  completed: boolean;
}

let todoInputEl: HTMLInputElement | null;
let todoListEl: HTMLUListElement | null;

let todos: Todo[] = [];

async function getTodos() {
  todos = (await invoke("get_todos")) as Todo[];
  renderTodos();
}

async function addTodo() {
  if (todoInputEl && todoInputEl.value.trim() !== "") {
    const newTodo = (await invoke("add_todo", {
      description: todoInputEl.value,
    })) as Todo;
    todos.push(newTodo);
    todoInputEl.value = "";
    renderTodos();
  }
}

async function toggleTodo(id: number) {
  const updatedTodo = (await invoke("toggle_todo", { id })) as Todo;
  if (updatedTodo) {
    todos = todos.map((todo) => (todo.id === id ? updatedTodo : todo));
    renderTodos();
  }
}

async function deleteTodo(id: number) {
  const deleted = (await invoke("delete_todo", { id })) as boolean;
  if (deleted) {
    todos = todos.filter((todo) => todo.id !== id);
    renderTodos();
  }
}

function renderTodos() {
  if (todoListEl) {
    todoListEl.innerHTML = "";
    todos.forEach((todo) => {
      const listItem = document.createElement("li");
      listItem.className = "todo-item";
      if (todo.completed) {
        listItem.classList.add("completed");
      }

      const checkbox = document.createElement("input");
      checkbox.type = "checkbox";
      checkbox.checked = todo.completed;
      checkbox.addEventListener("change", () => toggleTodo(todo.id));

      const descriptionSpan = document.createElement("span");
      descriptionSpan.textContent = todo.description;

      const deleteButton = document.createElement("button");
      deleteButton.textContent = "X";
      deleteButton.className = "delete-button";
      deleteButton.addEventListener("click", () => deleteTodo(todo.id));

      listItem.appendChild(checkbox);
      listItem.appendChild(descriptionSpan);
      listItem.appendChild(deleteButton);
      todoListEl!.appendChild(listItem);
    });
  }
}

window.addEventListener("DOMContentLoaded", () => {
  todoInputEl = document.querySelector("#todo-input");
  todoListEl = document.querySelector("#todo-list");

  document.querySelector("#todo-form")?.addEventListener("submit", (e) => {
    e.preventDefault();
    addTodo();
  });

  getTodos(); // Load todos on startup
});
