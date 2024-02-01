<div class="dialog p-5 w-full min-w-80 max-w-screen-md bg-white border rounded-lg shadow-md">
    <div class="header border-b">
        <h2> Header </h2>
    </div>
    <div class="top-content px-3">
        <div class="icon"/>
        <div class="details">
            <h3> Proxmox </h3>
            <div>
                Self-hosted Proxmox cluster
            </div>
        </div>
    </div>
    <form class="main-content">
        <label for="url"> URL: </label>
        <input name="url" id="url" type="text"/>

        <label for="title"> Title: </label>
        <input name="title" id="title" type="text"/>

        <label for="description"> Description: </label>
        <input name="description" id="description" type="text"/>

        <label for="shared"> Shared: </label>
        <input name="shared" id="shared" type="checkbox"/>

        <div class="form-buttons">
            <button> Cancel </button>
            <input name="submit" type="submit" value="Save" class="submit-button" />
        </div>
    </form>
</div>

<style>
 .dialog {
     margin: 0 20%;
     --dialog-border: 1px solid rgb(0, 0, 200, 0.5);
 }
 .header {
     width: 100%;
     padding: 8px 0;
     text-transform: uppercase;
 }
 .top-content {
     display: flex;
     flex-wrap: wrap;
     align-items: flex-start;
     padding: 10px;
     margin-left: 95px;
     gap: 10px;
 }

 .main-content {
     display: grid;
     gap: 4px;
     grid-template-columns: 100px 1fr;
 }

 .icon {
     width: 64px;
     height: 64px;
     border: 1px solid red;
 }

 label {
     text-align: end;
     padding: 4px;
 }
 input[type=text] {
     border: var(--dialog-border);
     border-radius: 3px;
     padding: 4px;
     width: 100%;
 }
 input[type=checkbox] {
     width: 30px;
 }
 input[type=submit] {
     background: #55f;
     border: var(--dialog-border);
     border-radius: 3px;
     color: white;
 }
 input:hover {
     border-color: rgb(0, 0, 200, 0.9);
 }
 
 .form-buttons {
     grid-column: span 2;
     margin-top: 10px;
     display: flex;
     flex-wrap: wrap;
     justify-content: flex-end;
     gap: 10px;
 }
 
 .form-buttons button {
     background: white;
     border: var(--dialog-border);
     border-radius: 3px;
 }
 
 .form-buttons > * {
     min-width: 100px;
 }
</style>
