import { Image } from "./styles";
import Logo from '../../images/logo.png';

const LogoWrapper = () => {
    return (
        <Image src={Logo} alt='logo' />
    );
};

export default LogoWrapper;