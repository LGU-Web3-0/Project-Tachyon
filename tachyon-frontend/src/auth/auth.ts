export namespace Auth {
    export interface LoginResult {
        success: boolean,
        message?: string,
        required_signature?: string
    }

    export async function login(email: string, password: string, signature?: string): Promise<LoginResult> {
        let request = {
            email,
            password,
            ...(signature ? {signature} : {})
        };

        const response = await window.fetch("/api/auth/login", {
            method: 'post',
            headers: {
                'content-type': 'application/json;charset=UTF-8',
            },
            body: JSON.stringify(request),
        });

        return await response.json()
    }
}





