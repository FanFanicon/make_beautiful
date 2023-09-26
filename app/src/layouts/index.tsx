import { Link, Outlet } from 'umi';
import styles from './index.less';

export default function Layout() {
  return (
    <div className={styles.navs}>
      <h1>我是layout</h1>
      <Outlet />
    </div>
  );
}
