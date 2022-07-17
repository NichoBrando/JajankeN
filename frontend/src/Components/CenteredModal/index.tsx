import React from 'react';
import { Modal } from './styles';

interface ICenteredModalProps {
    children: React.ReactNode;
};

const CenteredModal = ({ children }: ICenteredModalProps) => {
    return (
        <Modal>
            {children}
        </Modal>
    );
};

export default CenteredModal;