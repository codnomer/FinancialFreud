import { useState } from 'react'
import './App.css'
import { Input } from "@/components/ui/input"
import { Button } from "@/components/ui/button"

// Kullanıcı verisi arayüzü
interface NewUser {
  username: string;
  password_hash: string;
}

function App() {
  const [count, setCount] = useState(0);
  const [username, setUsername] = useState('');
  const [password, setPassword] = useState('');

  // Kullanıcı oluşturma işlemi
  const createUser = async () => {
    const userData: NewUser = {
      username,
      password_hash: password,
    };

    const response = await fetch('http://127.0.0.1:8080/', {
      method: 'GET',
      headers: {
        'Content-Type': 'application/json',
      },
      //body: JSON.stringify(userData),
    });

    if (response.ok) {
      const data = await response.json();
      console.log('User created:', data);
    } else {
      console.error('Error creating user');
    }
  };

  return (
    <div className="App">
      
      <div>
        <Input 
          type="text"
          value={username}
          onChange={(e) => setUsername(e.target.value)}
          placeholder="Username"
        />
        <Input
          type="password"
          value={password}
          onChange={(e) => setPassword(e.target.value)}
          placeholder="Password"
        />
        <Button onClick={createUser}>Create User</Button>
      </div>
    </div>
  );
}

export default App;
