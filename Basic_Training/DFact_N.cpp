#include<iostream>
using namespace std;
int fact(int n){
    int res=1;
    for(int i=1;i<=n;i++){
        res*=i;
    }
    return res;
}
int main(){
    int n;
    cout<<"Enter N : ";
    cin>>n;
    for(int i=1;i<n*2;i+=2){
        cout<<fact(i)<<" ";
        i+=2;
        if(i<n*2){
            cout<<-fact(i)<<" ";
        }

    }
    return 0;
}