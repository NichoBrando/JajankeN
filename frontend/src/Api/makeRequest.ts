import axios from 'axios';

const makeRequest = (url: string, method: string, body: any, useToken: boolean) => {
    const headers = {
        'Content-Type': 'application/json'
    };
    
    axios.defaults.headers.post['Content-Type'] = 'application/json';

    if (useToken) {
        headers['Authorization'] = localStorage.getItem('token');
    }
    
    return axios({
        url,
        method,
        withCredentials: false,
        headers,
        data: body,
    });
}

export default makeRequest;