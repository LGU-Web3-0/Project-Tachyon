export namespace Auth {
    export interface LoginResult {
        success: boolean,
        message?: string,
        signature_requirement?: string
    }

    export async function login_onclick() {
        const email = (document.getElementById("email") as HTMLInputElement).value
        const password = (document.getElementById("password") as HTMLInputElement).value
        let signature = undefined;
        if (!document.getElementById('signature').hidden) {
            signature = (document.getElementById("signature-input") as HTMLTextAreaElement).value
        }
        const result = await login(email, password, signature)

        if (result.success) {
            window.location.href = '/view/dashboard'
            return
        }

        if (result.message) {
            document.getElementById('message').hidden = false
            document.getElementById('message-content').textContent = result.message
        }

        if (result.signature_requirement) {
            document.getElementById('signature').hidden = false
            document.getElementById('signature-token').textContent = result.signature_requirement
        }
    }

    async function login(email: string, password: string, signature?: string): Promise<LoginResult> {
        let request = {
            email,
            password,
            ...(signature ? {signature} : {})
        };

        const response = await window.fetch("/api/user/login", {
            method: 'post',
            headers: {
                'content-type': 'application/json;charset=UTF-8',
            },
            body: JSON.stringify(request),
        });

        return await response.json()
    }

    export async function logout_onclick() {
        await window.fetch("/api/user/logout", {
            method: 'post',
            headers: {
                'content-type': 'application/json;charset=UTF-8',
            },
        });
        window.location.href = '/'
    }
}

export namespace UserView {
    export async function search_onclick() {
        const search_string = (document.getElementById("keywords") as HTMLInputElement).value;
        const page_size = (document.getElementById("pagesize") as HTMLInputElement).value;
        if (search_string.length > 0) {
            window.location.href = "/view/user?" + new URLSearchParams({search_string, page_size}).toString();
        } else {
            window.location.href = "/view/user?" + new URLSearchParams({page_size}).toString();
        }
    }
    export async function add_modal_trigger() {
        document.getElementById("add-user-modal").classList.remove("hidden");
    }
    export async function user_modal_cancel() {
        document.getElementById("add-user-modal").classList.add("hidden");
    }
    async function show_add_error(msg: string) {
        const message = document.getElementById("error-message");
        message.classList.remove("hidden");
        message.textContent = msg;
    }
    async function show_edit_error(msg: string) {
        const message = document.getElementById("user-edit-error");
        message.classList.remove("hidden");
        message.textContent = msg;
    }
    export async function user_addition() {
        const password = (document.getElementById("user-password") as HTMLInputElement).value;
        const confirmation = (document.getElementById("user-password-confirmation") as HTMLInputElement).value;
        if (password !== confirmation) {
            return await show_add_error("Passwords do not match");
        }
        const name = (document.getElementById("user-name") as HTMLInputElement).value;
        const email = (document.getElementById("user-email") as HTMLInputElement).value;
        const gpg_key = (document.getElementById("user-public-key") as HTMLInputElement).value;
        const request = {
            name,
            email,
            password,
            gpg_key,
        };
        const response = await window.fetch("/api/user/add", {
            method: 'post',
            headers: {
                'content-type': 'application/json;charset=UTF-8',
            },
            body: JSON.stringify(request),
        });
        const result = await response.json();
        if (result.success) {
            return document.location.reload();
        }
        if (result.message) {
            return await show_add_error(result.message);
        }
    }
    export async function user_lock(id: number) {
        const response = await window.fetch("/api/user/lock", {
            method: 'post',
            headers: {
                'content-type': 'application/json;charset=UTF-8',
            },
            body: JSON.stringify({id}),
        });
        if (response.status == 200) {
            return document.location.reload();
        }
    }
    export async function user_unlock(id: number) {
        const response = await window.fetch("/api/user/unlock", {
            method: 'post',
            headers: {
                'content-type': 'application/json;charset=UTF-8',
            },
            body: JSON.stringify({id}),
        });
        if (response.status == 200) {
            return document.location.reload();
        }
    }
    export async function user_edit_trigger(id: number) {
        (document.getElementById("user-edit-id") as HTMLInputElement).value = id.toString();
        document.getElementById("edit-user-modal").classList.remove("hidden");
    }
    export async function user_delete_trigger(id: number, name: string, email: string, fingerprint: string) {
        (document.getElementById("user-del-id") as HTMLInputElement).value = id.toString();
        (document.getElementById("user-del-name") as HTMLInputElement).value = name;
        (document.getElementById("user-del-email") as HTMLInputElement).value = email;
        (document.getElementById("user-del-fingerprint") as HTMLInputElement).value = fingerprint;
        document.getElementById("delete-user-modal").classList.remove("hidden");
    }
    export async function user_delete_cancel() {
        document.getElementById("delete-user-modal").classList.add("hidden");
    }
    export async function user_edit_cancel() {
        document.getElementById("edit-user-modal").classList.add("hidden");
    }
    export async function user_delete() {
        const id = (document.getElementById("user-del-id") as HTMLInputElement).value;
        const request = {
            id: parseInt(id),
        };
        const response = await window.fetch("/api/user/delete", {
            method: 'delete',
            headers: {
                'content-type': 'application/json;charset=UTF-8',
            },
            body: JSON.stringify(request),
        });
        if (response.status == 200) {
            return window.location.reload();
        }
    }
    export async function user_edit() {
        const password = (document.getElementById("user-edit-password") as HTMLInputElement);
        const confirmation = (document.getElementById("user-edit-password-confirmation") as HTMLInputElement);
        const name = (document.getElementById("user-edit-name") as HTMLInputElement);
        const email = (document.getElementById("user-edit-email") as HTMLInputElement);
        const pgp_key = (document.getElementById("user-edit-pubkey") as HTMLInputElement);
        const id = (document.getElementById("user-edit-id") as HTMLInputElement).value;
        if (password.value && password.value !== confirmation.value) {
            return await show_edit_error("Passwords do not match");
        }
        const request = {
            id: parseInt(id),
            ...(password.value ? {password: password.value} : {}),
            ...(name.value ? {name: name.value} : {}),
            ...(email.value ? {email: email.value} : {}),
            ...(pgp_key.value ? {pgp_key: pgp_key.value} : {}),
        };
        const response = await window.fetch("/api/user/edit", {
            method: 'post',
            headers: {
                'content-type': 'application/json;charset=UTF-8',
            },
            body: JSON.stringify(request),
        });
        if (response.status == 200) {
            return document.location.reload();
        }
        const json = await response.json();
        if (json.message) {
            return await show_edit_error(json.message);
        }
    }
}

export namespace Obj {
    export async function upload_onclick() {
        const form = document.getElementById("upload-form") as HTMLFormElement;
        const data = new FormData(form);
        const request = new XMLHttpRequest()
        request.open("POST", "/api/object/upload")
        request.onreadystatechange = function () {
            if (request.readyState === 4) {
                const result = JSON.parse(request.responseText)
                const message = document.getElementById("form-message") as HTMLParagraphElement;
                if (result.success) {
                    message.hidden = false;
                    message.textContent = "Uploaded successfully!";
                    message.classList.remove("text-red-500", "text-sm");
                    message.classList.add("text-sm", "text-green-500");
                    setTimeout(() => {
                        window.location.reload();
                    }, 1000);
                } else {
                    message.hidden = false;
                    message.textContent = "Upload failed: " + result.message;
                    message.classList.remove("text-green-500", "text-sm");
                    message.classList.add("text-sm", "text-red-500");
                }
            }
        }
        request.send(data)
    }

    export async function change_visibility_onclick(uuid: String) {
        const request = {
            uuid
        };
        const response = await window.fetch("/api/object/visibility", {
            method: 'post',
            headers: {
                'content-type': 'application/json;charset=UTF-8',
            },
            body: JSON.stringify(request),
        });
        const json = await response.json();
        if (json.success) {
            window.location.reload();
        }
    }

    export async function delete_onclick(uuid: String) {
        const request = {
            uuid
        };
        const response = await window.fetch("/api/object/delete", {
            method: 'post',
            headers: {
                'content-type': 'application/json;charset=UTF-8',
            },
            body: JSON.stringify(request),
        });
        const json = await response.json();
        if (json.success) {
            window.location.reload();
        }
    }
}

export namespace Task {

	export interface AddTaskResult {
		success: boolean,
		message?: string
	}

	export async function add_task_onclick() {
		document.getElementById("add-task-modal").classList.remove("hidden")
	}

	export async function cancel_task_onclick() {
		document.getElementById("add-task-modal").classList.add("hidden")
	}

	export async function really_add_task_onclick() {
         const task_name = (document.getElementById("task-name") as HTMLInputElement).value     
		 const create_time = new Date().toISOString();
         const due_time = (document.getElementById("due-time") as HTMLInputElement).valueAsDate.toISOString()
         const task_description = (document.getElementById("task-description") as HTMLInputElement).value   
		 const result = await add_task(task_name, create_time, due_time, task_description)
		 if (result.success) {
			 document.getElementById("add-task-modal").classList.add("hidden")
			 window.location.reload()
		 }
		 if (!result.success){
			 document.getElementById("add-task-modal").classList.add("hidden")
		 }
	}	

	async function add_task(task_name: string, create_time: string, due_time: string, task_description: string): Promise<AddTaskResult>{
	     let request = {
              name: task_name,
			  create_date: create_time,
			  due_date: due_time,
			  description: task_description,
		 };

		 const response = await window.fetch("/api/task/add", {
			 method: 'post',
			 headers: {
				 'content-type': 'application/json;charset=UTF-8',
			 },
			 body: JSON.stringify(request),
		 });

		 return await response.json()
											
	}
}

export function test() {
    console.log('test')
}
