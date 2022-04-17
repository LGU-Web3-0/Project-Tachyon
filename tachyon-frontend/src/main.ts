export namespace Auth {
    export interface LoginResult {
        success: boolean,
        message?: string,
        signature_requirement?: string
    }


	export async function add_task_onclick() {
		document.getElementById("add-task-modal").classList.remove("hidden")
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
                    message.textContent = "Upload successful!";
                    message.classList.remove("text-red-500", "text-sm");
                    message.classList.add("text-sm", "text-green-500");
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

export function test() {
    console.log('test')
}

  
  

  
  
