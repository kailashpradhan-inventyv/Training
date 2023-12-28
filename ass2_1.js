function combination(n,r){
    return fact(n)/(fact(r)*fact(n-r));
}
function fact(r){
    if(r==1){
        return 1;
    }
    else{
        return r*fact(r-1);
    }
}
function ncr(){
    const n=10;
    const r=3;
    return combination(n,r)
}
console.log(ncr());