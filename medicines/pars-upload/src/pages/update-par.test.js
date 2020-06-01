import ParUpload from './update-par'

describe(ParUpload, () => {
  it('extracts ID of par to update', () => {
    let step1 = { type: 'product', data: new FormData() }
    let step2 = { type: 'file', data: new FormData() }
    let step3 = { type: 'review', data: new FormData() }
    let step4 = { type: 'get_par', data: new FormData() }
    let parId = '17dfsjkl12kljdfk129'
    step4.data.append('par_url', `https://blob.net/${parId}`)

    let id = ParUpload.getIdOfParToUpdate([step1, step2, step3])
    expect(id).toBe(parId)
  })
  // it('should render 🚫', () => {
  //   // tslint:disable-next-line: no-empty
  //   const noop = () => {};
  //   const component = shallow(
  //     <CookieForm storageAllowed={false} setStorageAllowed={noop} />,
  //   );
  //   expect(component).toMatchSnapshot();
  // });
})
