import makeRequest from './makeRequest';

export const Login = async (username: string, password: string) => {
    const response = await makeRequest(
        `http://localhost:8000/user/login`,
        'POST',
        { username, password },
        false
    );

    localStorage.setItem('token', response.data);
}

export const SignUp = async (username: string, password: string, display_name: string) => {
    await makeRequest(
        `http://localhost:8000/user`,
        'POST',
        { username, password, display_name},
        false
    );

    Login(username, password);
}