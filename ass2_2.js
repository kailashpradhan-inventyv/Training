const arr=[1,2,3,4,5];
function array(arr,arr2=[],len=0){
    if(len<arr.length){
        arr2.push(arr[len]);
        array(arr,arr2,len+1);
    }
    return arr2;
}
const arr2=array(arr)
function displayArr(arr2,len=0){
    if(len<arr2.length){
        console.log(arr2[len]);
        displayArr(arr2,len+1);
    }
}
displayArr(arr2);
