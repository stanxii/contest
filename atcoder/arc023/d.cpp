#include <vector>
#include <cassert>
/*
 * Sparse Table.
 * BiOp should be the type of a binary operator which is
 * associative, commutative and idempotent.
 * Header Requirement: vector, cassert
 * Verified by: AtCoder ARC023 D (http://arc023.contest.atcoder.jp/submissions/960757)
 */
template<class T, class BiOp>
class SparseTable {
private:
  BiOp biop;
  std::vector<std::vector<int> > st;
  void create_sparse_table(int n, const std::vector<int> &lcp) {
    int h = 1;
    while ((1 << h) < n) {
      ++h;
    }
    st = std::vector<std::vector<int> >(h + 1, std::vector<int>(n));

    for (int i = 0; i < n; ++i) {
      st[0][i] = lcp[i];
    }
    for (int j = 1; j <= h; ++j) {
      for (int i = 0; i <= n - (1 << j); ++i) {
	st[j][i] = biop(st[j - 1][i], st[j - 1][i + (1 << (j-1))]);
      }
    }
  }
  /*
   * Reference: https://graphics.stanford.edu/~seander/bithacks.html#IntegerLogFloat
   */
  static int top_bit(int t) {
    const float v = t; // find int(log2(v)), where v > 0.0 && finite(v) && isnormal(v)
    int c;         // 32-bit int c gets the result;
    
    c = *(const int *) &v;  // OR, for portability:  memcpy(&c, &v, sizeof c);
    return (c >> 23) - 127;
  }
public:
  /*
   * Initializes this sparse table. O(n log n) where n = ary.size().
   */
  SparseTable(BiOp biop, const std::vector<int> &ary): biop(biop) {
    create_sparse_table(ary.size(), ary);
  }
  /*
   * Computes biop(ary[f], ary[f+1], ..., ary[s]). O(1).
   * Note: the interval is inclusive.
   */
  int query(int f, int s) const {
    assert (f <= s);
    int diff = top_bit(s + 1 - f);
    return biop(st[diff][f], st[diff][s + 1 - (1 << diff)]);
  }
};

#include <iostream>
#include <map>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
const double EPS=1e-9;

const int N=100000;
int n,m;
int x[N];
inline int gcd_core(int x, int y) {
  while (y != 0) {
    int r = x % y;
    x = y;
    y = r;
  }
  return x;
}
struct gcd {
  int operator()(int x, int y) const {
    return gcd_core(x, y);
  }
};

inline void add(map<int, ll> &freq, int val, int f) {
  if (freq.count(val)) {
    freq[val] += f;
    return;
  }
  freq[val] = f;
}

int main(void){
  cin>>n>>m;
  vector<int> a(n);
  REP(i,0,n){
    cin>>a[i];
  }
  REP(i,0,m){
    cin>>x[i];
  }
  SparseTable<int, gcd> sp_tbl(gcd(), a);
  map<int, ll> freq;
  REP(i, 0, n - 1) { // [i, j] for i < j
    int pos = i + 1;
    int cur = gcd_core(a[i], a[i + 1]);
    // cur == sp_tbl.query(i, pos);
    while (pos < n) {
      int lo = pos, hi = n;
      while (hi - lo > 1) {
	int mid = (hi + lo) / 2;
	if (sp_tbl.query(i, mid) == cur) {
	  lo = mid;
	} else {
	  hi = mid;
	}
      }
      add(freq, cur, hi - pos);
      pos = hi;
      cur = gcd_core(cur, a[hi]);
    }
  }
  REP(i, 0, n) {
    add(freq, a[i], 1); // [i, i]
  }
  REP(i, 0, m) {
    cout << freq[x[i]] << endl;
  }
}
