import React from 'react';
import { Input } from '../Input';

const LoginForm: React.FC = ({ form, setForm }) => {
    return (
        <div>
            <Input 
                type="text" 
                onChange={({ target }) => {
                    setForm({
                        ...form,
                        username: target.value.trim()
                    })
                }}
                placeholder="Username"
                value={form.username}
            />
            <Input 
                type="password" 
                placeholder="Password"
                onChange={({ target }) => {
                    setForm({
                        ...form,
                        password: target.value.trim()
                    })
                }}
            />
        </div>
    );
};

export default LoginForm;