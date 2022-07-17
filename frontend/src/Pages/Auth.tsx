import React, { useState } from 'react';
import ActionButton from '../Components/ActionButton';
import CenteredModal from '../Components/CenteredModal';
import LoginForm from '../Components/LoginForm';
import LogoWrapper from '../Components/LogoWrapper';
import SignUpForm from '../Components/SignUpForm';
import { toast } from 'react-toastify';
import { Login, SignUp } from '../Api/Auth';
import { useNavigate } from 'react-router-dom';

const Auth: React.FC = () => {
    const navigate = useNavigate();

    const [isLogin, setIsLogin] = useState(true);
    const [form, setForm] = useState({
        username: '',
        password: '',
        display_name: ''
    });
    const [isLoading, setIsLoading] = useState(false);

    const changeFormMethod = () => {
        if (isLoading) return;
        setIsLogin(!isLogin);
    };

    const redirectToHome = () => {
        navigate('/');
    }

    const handleSubmit = async () => {
        if (isLoading) return;
        if (form.username.length < 6) {
            toast('Username must be at least 6 characters long', {
                type: 'error'
            });
        }
        if (form.password.length < 5) {
            toast('Password must be at least 5 characters long', {
                type: 'error'
            });
        }

        if (form.display_name.length < 3 && !isLogin) {
            toast('Display name must be at least 3 characters long', {
                type: 'error'
            });
        }

        try {
            setIsLoading(true);
            if (isLogin) {
                await Login(form.username, form.password);  
                redirectToHome();              
            } else {
                await SignUp(form.username, form.password, form.display_name);
                redirectToHome();             
            }
        }
        catch (err) {
            console.error(err);
            toast(err?.data?.message || 'Something went wrong', {
                type: 'error'
            });
        }
        finally {
            setIsLoading(false);
        }
    };

    return (
        <CenteredModal>
            <LogoWrapper />

            {isLogin ? <LoginForm form={form} setForm={setForm} /> : <SignUpForm form={form} setForm={setForm} />}

            <a onClick={changeFormMethod}>{isLogin ? 'Sign Up' : 'Login'}</a>

            <ActionButton onClick={handleSubmit}>
                {isLogin ? 'Login' : 'Sign Up'}
            </ActionButton>
        </CenteredModal>
    );
}

export default Auth;