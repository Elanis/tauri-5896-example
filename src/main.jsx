import React from 'react';
import ReactDOM from 'react-dom/client';

import { convertFileSrc } from '@tauri-apps/api/tauri';

import './index.css';

const root = ReactDOM.createRoot(document.getElementById('root'));
root.render(
	<img src={convertFileSrc('assets/img/achievements.png')} />
);
