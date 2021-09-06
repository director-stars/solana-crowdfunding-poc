import {Wallet} from './components/wallet';
import logo from './logo.svg';
import './App.css';

function App() {
  return (
    <div className="App">
      <div className="d-flex flex-column flex-md-row align-items-center p-3 px-md-4 mb-3 bg-white border-bottom box-shadow">
        <h5 className="my-0 mr-md-auto font-weight-normal">WEBDEV_PHASE1</h5>
        <nav className="my-2 my-md-0 mr-md-3">
        </nav>
        <Wallet/>
      </div>

      <div className="pricing-header px-3 py-3 pt-md-5 pb-md-4 mx-auto text-center">
        <h1 className="display-4">NFT Purchase</h1>
        <p className="lead">Quickly interact with NFT minter contract.</p>
      </div>

      <div className="container">
        <div className="card-deck mb-3 text-center">
          <div className="card mb-4 box-shadow">
            <div className="card-header">
              <h4 className="my-0 font-weight-normal">Purchase</h4>
            </div>
            <div className="card-body">
              <h1 className="card-title pricing-card-title">2 SOL <small className="text-muted">/ NFT</small></h1>
              <ul className="list-unstyled mt-3 mb-4">
                <li>... </li>
                <li>... </li>
                <li> ...</li>
                <li>... </li>
              </ul>
              <button type="button" className="btn btn-lg btn-block btn-primary">BUY</button>
            </div>
          </div>
          <div className="card mb-4 box-shadow">
            <div className="card-header">
              <h4 className="my-0 font-weight-normal">FundMove</h4>
            </div>
            <div className="card-body">
              <h1 className="card-title pricing-card-title">Move All</h1>
              <ul className="list-unstyled mt-3 mb-4">
                <li>...</li>
                <li>...</li>
                <li>...</li>
                <li>...</li>
              </ul>
              <button type="button" className="btn btn-lg btn-block btn-primary">WITHDRAW</button>
            </div>
          </div>
        </div>

        <footer className="pt-4 my-md-5 pt-md-5 border-top">
          <div className="row">
            <div className="col-12 col-md">
              <img className="mb-2" src="./logo.ico" alt="" width="24" height="24"/>
              <small className="d-block mb-3 text-muted">&copy; 2021</small>
            </div>
          </div>
        </footer>
      </div>
    </div>
  );
}

export default App;
