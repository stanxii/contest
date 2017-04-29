#include <algorithm>
#include <bitset>
#include <cassert>
#include <cctype>
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <ctime>
#include <deque>
#include <functional>
#include <iomanip>
#include <iostream>
#include <list>
#include <map>
#include <numeric>
#include <queue>
#include <set>
#include <sstream>
#include <stack>
#include <string>
#include <utility>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;
const ll mod = 1e9 + 7;



int main(void){
  int n;
  ll t;
  cin >> n >> t;
  vector<ll> ts(n);
  REP(i, 0, n) {
    cin >> ts[i];
  }
  ll tot = 0;
  REP(i, 0, n - 1) {
    tot += min(t, ts[i + 1]- ts[i]);
  }
  tot += t;
  cout << tot << endl;
}
