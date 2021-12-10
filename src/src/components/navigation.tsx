import {
  BrowserRouter as Router,
  Route,
  NavLink,
  Routes
} from "react-router-dom";
import AddNews from "./addnews";
import News from "./news";
import Wallet from "./wallet";


function Navigation() {
  return (
    <div>
      <Router>
        <div>
          <ul className="menu" >
            <li>
              <NavLink to="/news" className={({ isActive }) => (isActive ? 'active' : 'inactive')}  >News</NavLink>
            </li>
            <li>
              <NavLink to="/addnews" className={({ isActive }) => (isActive ? 'active' : 'inactive')}>Add news</NavLink>
            </li>
            <li>
              <NavLink to="/wallet" className={({ isActive }) => (isActive ? 'active' : 'inactive')} >Wallet</NavLink>
            </li>
          </ul>
        </div>
        <Routes>
          <Route path="/news" element={<News/>}/>
          <Route path="/addnews" element={<AddNews/>}/>
          <Route path="/wallet" element={<Wallet/>}/>
        </Routes>
      </Router>
    </div>
  );
}

export default Navigation;