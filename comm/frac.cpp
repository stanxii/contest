typedef long long int ll;

ll gcd(ll x,ll y){
  while(y!=0){
    ll r=x%y;
    x=y;y=r;
  }
  return x;
}

struct frac{
  ll x,y;frac(ll x,ll y):x(x),y(y){
    reduce();
  }
  frac operator+(const frac& f)const{
    return frac(x*f.y+f.x*y, y*f.y);
  }
  frac operator*(const frac& f)const{
    return frac(x*f.x,y*f.y);
  }
  bool operator<(const frac &f) const {
    return x*f.y < y*f.x;
  }
  bool operator==(const frac &f) const {
    return x*f.y == y*f.x;
  }
  void reduce(){
    ll g=gcd(x,y);
    x/=g;
    y/=g;
    if(y<0){
      x=-x;y=-y;
    }
  }
};
