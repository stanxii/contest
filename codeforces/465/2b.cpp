#include <algorithm>
#include <cassert>
#include <cctype>
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <ctime>
#include <deque>
#include <functional>
#include <iomanip>
#include <iostream>
#include <list>
#include <map>
#include <queue>
#include <random>
#include <set>
#include <sstream>
#include <string>
#include <utility>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)
#define DEBUGP(val) cerr << #val << "=" << val << "\n"

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;
const ll mod = 1e9 + 7;



int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n;
  string s;
  cin >> n >> s;
  int x = 0, sgn = 0;
  int tot = 0;
  REP(i, 0, n) {
    if (s[i] == 'U') {
      x += 1;
    } else {
      x -= 1;
    }
    if (x != 0 && sgn != 0 && x * sgn < 0) {
      tot += 1;
    }
    if (x > 0) {
      sgn = 1;
    }
    if (x < 0) {
      sgn = -1;
    }
  }
  cout << tot << "\n";
}
